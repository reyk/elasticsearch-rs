use inflector::Inflector;
use quote::{ToTokens, Tokens};

use super::{ok_or_accumulate, Step};
use api_generator::generator::{Api, ApiEndpoint, TypeKind};
use itertools::Itertools;
use std::collections::BTreeMap;
use yaml_rust::{yaml::Hash, Yaml, YamlEmitter};

pub struct Do {
    headers: BTreeMap<String, String>,
    catch: Option<String>,
    pub api_call: ApiCall,
    warnings: Vec<String>,
}

impl ToTokens for Do {
    fn to_tokens(&self, tokens: &mut Tokens) {
        // TODO: Add in catch, headers, warnings
        &self.api_call.to_tokens(tokens);
    }
}

impl From<Do> for Step {
    fn from(d: Do) -> Self {
        Step::Do(d)
    }
}

impl Do {
    pub fn try_parse(api: &Api, yaml: &Yaml) -> Result<Do, failure::Error> {
        let hash = yaml
            .as_hash()
            .ok_or_else(|| failure::err_msg(format!("Expected hash but found {:?}", yaml)))?;

        let mut api_call: Option<ApiCall> = None;
        let mut headers = BTreeMap::new();
        let mut warnings: Vec<String> = Vec::new();
        let mut catch = None;

        let results: Vec<Result<(), failure::Error>> = hash
            .iter()
            .map(|(k, v)| {
                let key = k.as_str().ok_or_else(|| {
                    failure::err_msg(format!("expected string key but found {:?}", k))
                })?;

                match key {
                    "headers" => {
                        match v.as_hash() {
                            Some(h) => {
                                //for (k, v) in h.iter() {}

                                Ok(())
                            }
                            None => {
                                Err(failure::err_msg(format!("expected hash but found {:?}", v)))
                            }
                        }
                    }
                    "catch" => {
                        catch = v.as_str().map(|s| s.to_string());
                        Ok(())
                    }
                    "node_selector" => {
                        // TODO: implement
                        Ok(())
                    }
                    "warnings" => {
                        warnings = v
                            .as_vec()
                            .map(|a| a.iter().map(|y| y.as_str().unwrap().to_string()).collect())
                            .unwrap();
                        Ok(())
                    }
                    call => {
                        let hash = v.as_hash().ok_or_else(|| {
                            failure::err_msg(format!(
                                "expected hash value for {} but found {:?}",
                                &call, v
                            ))
                        })?;

                        let endpoint = api.endpoint_for_api_call(call).ok_or_else(|| {
                            failure::err_msg(format!("no API found for {}", call))
                        })?;

                        api_call = Some(ApiCall::try_from(api, endpoint, hash)?);
                        Ok(())
                    }
                }
            })
            .collect();

        ok_or_accumulate(&results, 0)?;

        Ok(Do {
            api_call: api_call.unwrap(),
            catch,
            headers,
            warnings,
        })
    }
}

/// The components of an API call
pub struct ApiCall {
    pub namespace: Option<String>,
    function: syn::Ident,
    parts: Option<Tokens>,
    params: Option<Tokens>,
    body: Option<Tokens>,
    ignore: Option<i64>,
}

impl ToTokens for ApiCall {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let function = &self.function;
        let parts = &self.parts;
        let params = &self.params;
        let body = &self.body;

        tokens.append(quote! {
            let response = client.#function(#parts)
                #params
                #body
                .send()
                .await?;
        });
    }
}

impl ApiCall {
    /// Try to create an API call
    pub fn try_from(
        api: &Api,
        endpoint: &ApiEndpoint,
        hash: &Hash,
    ) -> Result<ApiCall, failure::Error> {
        let mut parts: Vec<(&str, &Yaml)> = vec![];
        let mut params: Vec<(&str, &Yaml)> = vec![];
        let mut body: Option<Tokens> = None;
        let mut ignore: Option<i64> = None;

        // work out what's a URL part and what's a param in the supplied
        // arguments for the API call
        for (k, v) in hash.iter() {
            let key = k.as_str().unwrap();
            if endpoint.params.contains_key(key) || api.common_params.contains_key(key) {
                params.push((key, v));
            } else if key == "body" {
                body = Self::generate_body(endpoint, v);
            } else if key == "ignore" {
                ignore = match v.as_i64() {
                    Some(i) => Some(i),
                    // handle ignore as an array of i64
                    None => v.as_vec().unwrap()[0].as_i64(),
                }
            } else {
                parts.push((key, v));
            }
        }

        let api_call = endpoint.full_name.as_ref().unwrap();
        let parts = Self::generate_parts(api_call, endpoint, &parts)?;
        let params = Self::generate_params(api, endpoint, &params)?;
        let function = syn::Ident::from(api_call.replace(".", "()."));
        let namespace: Option<String> = if api_call.contains('.') {
            let namespaces: Vec<&str> = api_call.splitn(2, '.').collect();
            Some(namespaces[0].to_string())
        } else {
            None
        };

        Ok(ApiCall {
            namespace,
            function,
            parts,
            params,
            body,
            ignore,
        })
    }

    fn generate_params(
        api: &Api,
        endpoint: &ApiEndpoint,
        params: &[(&str, &Yaml)],
    ) -> Result<Option<Tokens>, failure::Error> {
        match params.len() {
            0 => Ok(None),
            _ => {
                let mut tokens = Tokens::new();
                for (n, v) in params {
                    let param_ident =
                        syn::Ident::from(api_generator::generator::code_gen::valid_name(n));

                    let ty = match endpoint.params.get(*n) {
                        Some(t) => Ok(t),
                        None => match api.common_params.get(*n) {
                            Some(t) => Ok(t),
                            None => Err(failure::err_msg(format!("No param found for {}", n))),
                        },
                    }?;

                    let kind = ty.ty;

                    fn create_enum(
                        enum_name: &str,
                        variant: &str,
                        options: &[serde_json::Value],
                    ) -> Result<Tokens, failure::Error> {
                        if !variant.is_empty()
                            && !options.contains(&serde_json::Value::String(variant.to_owned()))
                        {
                            return Err(failure::err_msg(format!(
                                "options {:?} does not contain value {}",
                                &options, variant
                            )));
                        }

                        let e: String = enum_name.to_pascal_case();
                        let enum_name = syn::Ident::from(e.as_str());
                        let variant = if variant.is_empty() {
                            // TODO: Should we simply omit empty Refresh tests?
                            if e == "Refresh" {
                                syn::Ident::from("True")
                            } else if e == "Size" {
                                syn::Ident::from("Unspecified")
                            } else {
                                return Err(failure::err_msg(format!(
                                    "Unhandled empty value for {}",
                                    &e
                                )));
                            }
                        } else {
                            syn::Ident::from(variant.to_pascal_case())
                        };

                        Ok(quote!(#enum_name::#variant))
                    }

                    match v {
                        Yaml::String(ref s) => {
                            match kind {
                                TypeKind::Enum => {
                                    if n == &"expand_wildcards" {
                                        // expand_wildcards might be defined as a comma-separated
                                        // string. e.g.
                                        let idents: Vec<Result<Tokens, failure::Error>> = s
                                            .split(',')
                                            .collect::<Vec<_>>()
                                            .iter()
                                            .map(|e| create_enum(n, e, &ty.options))
                                            .collect();

                                        match ok_or_accumulate(&idents, 0) {
                                            Ok(_) => {
                                                let idents: Vec<Tokens> = idents
                                                    .into_iter()
                                                    .filter_map(Result::ok)
                                                    .collect();

                                                tokens.append(quote! {
                                                    .#param_ident(&[#(#idents),*])
                                                });
                                            }
                                            Err(e) => return Err(failure::err_msg(e)),
                                        }
                                    } else {
                                        let e = create_enum(n, s.as_str(), &ty.options)?;
                                        tokens.append(quote! {
                                            .#param_ident(#e)
                                        });
                                    }
                                }
                                TypeKind::List => {
                                    let values: Vec<&str> = s.split(',').collect();
                                    tokens.append(quote! {
                                        .#param_ident(&[#(#values),*])
                                    })
                                }
                                TypeKind::Boolean => {
                                    let b = s.parse::<bool>()?;
                                    tokens.append(quote! {
                                        .#param_ident(#b)
                                    });
                                }
                                TypeKind::Double => {
                                    let f = s.parse::<f64>()?;
                                    tokens.append(quote! {
                                        .#param_ident(#f)
                                    });
                                }
                                TypeKind::Integer | TypeKind::Number => {
                                    let i = s.parse::<i32>()?;
                                    tokens.append(quote! {
                                        .#param_ident(#i)
                                    });
                                }
                                _ => tokens.append(quote! {
                                    .#param_ident(#s)
                                }),
                            }
                        }
                        Yaml::Boolean(ref b) => match kind {
                            TypeKind::Enum => {
                                let enum_name = syn::Ident::from(n.to_pascal_case());
                                let variant = syn::Ident::from(b.to_string().to_pascal_case());
                                tokens.append(quote! {
                                    .#param_ident(#enum_name::#variant)
                                })
                            }
                            TypeKind::List => {
                                // TODO: _source filter can be true|false|list of strings
                                let s = b.to_string();
                                tokens.append(quote! {
                                    .#param_ident(&[#s])
                                })
                            }
                            _ => {
                                tokens.append(quote! {
                                    .#param_ident(#b)
                                });
                            }
                        },
                        Yaml::Integer(ref i) => match kind {
                            TypeKind::String => {
                                let s = i.to_string();
                                tokens.append(quote! {
                                    .#param_ident(#s)
                                })
                            }
                            TypeKind::Integer => {
                                // yaml-rust parses all as i64
                                let int = *i as i32;
                                tokens.append(quote! {
                                    .#param_ident(#int)
                                });
                            }
                            TypeKind::Float => {
                                // yaml-rust parses all as i64
                                let f = *i as f32;
                                tokens.append(quote! {
                                    .#param_ident(#f)
                                });
                            }
                            TypeKind::Double => {
                                // yaml-rust parses all as i64
                                let f = *i as f64;
                                tokens.append(quote! {
                                    .#param_ident(#f)
                                });
                            }
                            _ => {
                                tokens.append(quote! {
                                    .#param_ident(#i)
                                });
                            }
                        },
                        Yaml::Array(arr) => {
                            // only support param string arrays
                            let result: Vec<&String> = arr
                                .iter()
                                .map(|i| match i {
                                    Yaml::String(s) => Ok(s),
                                    y => Err(failure::err_msg(format!(
                                        "Unsupported array value {:?}",
                                        y
                                    ))),
                                })
                                .filter_map(Result::ok)
                                .collect();

                            if n == &"expand_wildcards" {
                                let result: Vec<Result<Tokens, failure::Error>> = result
                                    .iter()
                                    .map(|s| create_enum(n, s.as_str(), &ty.options))
                                    .collect();

                                match ok_or_accumulate(&result, 0) {
                                    Ok(_) => {
                                        let result: Vec<Tokens> =
                                            result.into_iter().filter_map(Result::ok).collect();

                                        tokens.append(quote! {
                                            .#param_ident(&[#(#result),*])
                                        });
                                    }
                                    Err(e) => return Err(failure::err_msg(e)),
                                }
                            } else {
                                tokens.append(quote! {
                                    .#param_ident(&[#(#result),*])
                                });
                            }
                        }
                        _ => println!("Unsupported value {:?}", v),
                    }
                }

                Ok(Some(tokens))
            }
        }
    }

    fn generate_parts(
        api_call: &str,
        endpoint: &ApiEndpoint,
        parts: &[(&str, &Yaml)],
    ) -> Result<Option<Tokens>, failure::Error> {
        // TODO: ideally, this should share the logic from EnumBuilder
        let enum_name = {
            let name = api_call.to_pascal_case().replace(".", "");
            syn::Ident::from(format!("{}Parts", name))
        };

        // Enum variants containing no URL parts where there is only a single API URL,
        // are not required to be passed in the API
        if parts.is_empty() {
            let param_counts = endpoint
                .url
                .paths
                .iter()
                .map(|p| p.path.params().len())
                .collect::<Vec<usize>>();

            if !param_counts.contains(&0) {
                return Err(failure::err_msg(format!(
                    "No path for '{}' API with no URL parts",
                    api_call
                )));
            }

            return match endpoint.url.paths.len() {
                1 => Ok(None),
                _ => Ok(Some(quote!(#enum_name::None))),
            };
        }

        let path = match endpoint.url.paths.len() {
            1 => {
                let path = &endpoint.url.paths[0];
                if path.path.params().len() == parts.len() {
                    Some(path)
                } else {
                    None
                }
            }
            _ => {
                // get the matching path parts
                let matching_path_parts = endpoint
                    .url
                    .paths
                    .iter()
                    .filter(|path| {
                        let p = path.path.params();
                        if p.len() != parts.len() {
                            return false;
                        }

                        let contains = parts
                            .iter()
                            .filter_map(|i| if p.contains(&i.0) { Some(()) } else { None })
                            .collect::<Vec<_>>();
                        contains.len() == parts.len()
                    })
                    .collect::<Vec<_>>();

                match matching_path_parts.len() {
                    0 => None,
                    _ => Some(matching_path_parts[0]),
                }
            }
        };

        if path.is_none() {
            return Err(failure::err_msg(format!(
                "No path for '{}' API with URL parts {:?}",
                &api_call, parts
            )));
        }

        let path = path.unwrap();
        let path_parts = path.path.params();
        let variant_name = {
            let v = path_parts
                .iter()
                .map(|k| k.to_pascal_case())
                .collect::<Vec<_>>()
                .join("");
            syn::Ident::from(v)
        };

        let part_tokens: Vec<Result<Tokens, failure::Error>> = parts
            .iter()
            // don't rely on URL parts being ordered in the yaml test
            .sorted_by(|(p, _), (p2, _)| {
                let f = path_parts.iter().position(|x| x == p).unwrap();
                let s = path_parts.iter().position(|x| x == p2).unwrap();
                f.cmp(&s)
            })
            .map(|(p, v)| {
                let ty = match path.parts.get(*p) {
                    Some(t) => Ok(t),
                    None => Err(failure::err_msg(format!(
                        "No URL part found for {} in {}",
                        p, &path.path
                    ))),
                }?;

                match v {
                    Yaml::String(s) => match ty.ty {
                        TypeKind::List => {
                            let values: Vec<&str> = s.split(',').collect();
                            Ok(quote! { &[#(#values),*] })
                        }
                        TypeKind::Long => {
                            let l = s.parse::<i64>().unwrap();
                            Ok(quote! { #l })
                        }
                        _ => Ok(quote! { #s }),
                    },
                    Yaml::Boolean(b) => {
                        let s = b.to_string();
                        Ok(quote! { #s })
                    }
                    Yaml::Integer(i) => match ty.ty {
                        TypeKind::Long => Ok(quote! { #i }),
                        _ => {
                            let s = i.to_string();
                            Ok(quote! { #s })
                        }
                    },
                    Yaml::Array(arr) => {
                        // only support param string arrays
                        let result: Vec<_> = arr
                            .iter()
                            .map(|i| match i {
                                Yaml::String(s) => Ok(s),
                                y => Err(failure::err_msg(format!(
                                    "Unsupported array value {:?}",
                                    y
                                ))),
                            })
                            .collect();

                        match ok_or_accumulate(&result, 0) {
                            Ok(_) => {
                                let result: Vec<_> =
                                    result.into_iter().filter_map(Result::ok).collect();

                                match ty.ty {
                                    // Some APIs specify a part is a string in the REST API spec
                                    // but is really a list, which is what a YAML test might pass
                                    // e.g. security.get_role_mapping.
                                    // see https://github.com/elastic/elasticsearch/pull/53207
                                    TypeKind::String => {
                                        let s = result.iter().join(",");
                                        Ok(quote! { #s })
                                    }
                                    _ => Ok(quote! { &[#(#result),*] }),
                                }
                            }
                            Err(e) => Err(failure::err_msg(e)),
                        }
                    }
                    _ => Err(failure::err_msg(format!("Unsupported value {:?}", v))),
                }
            })
            .collect();

        match ok_or_accumulate(&part_tokens, 0) {
            Ok(_) => {
                let part_tokens: Vec<Tokens> =
                    part_tokens.into_iter().filter_map(Result::ok).collect();
                Ok(Some(
                    quote! { #enum_name::#variant_name(#(#part_tokens),*) },
                ))
            }
            Err(e) => Err(failure::err_msg(e)),
        }
    }

    /// Creates the body function call from a YAML value.
    ///
    /// When reading a body from the YAML test, it'll be converted to a Yaml variant,
    /// usually a Hash. To get the JSON representation back requires converting
    /// back to JSON
    fn generate_body(endpoint: &ApiEndpoint, v: &Yaml) -> Option<Tokens> {
        let accepts_nd_body = match &endpoint.body {
            Some(b) => match &b.serialize {
                Some(s) => s == "bulk",
                _ => false,
            },
            None => false,
        };

        match v {
            Yaml::String(s) => {
                if accepts_nd_body {
                    Some(quote!(.body(vec![#s])))
                } else {
                    Some(quote!(.body(#s)))
                }
            }
            _ => {
                let mut s = String::new();
                {
                    let mut emitter = YamlEmitter::new(&mut s);
                    emitter.dump(v).unwrap();
                }

                if accepts_nd_body {
                    let values: Vec<serde_json::Value> = serde_yaml::from_str(&s).unwrap();
                    let json: Vec<Tokens> = values
                        .iter()
                        .map(|value| {
                            let json = serde_json::to_string(&value).unwrap();
                            let ident = syn::Ident::from(json);
                            if value.is_string() {
                                quote!(#ident)
                            } else {
                                quote!(JsonBody::from(json!(#ident)))
                            }
                        })
                        .collect();
                    Some(quote!(.body(vec![ #(#json),* ])))
                } else {
                    let value: serde_json::Value = serde_yaml::from_str(&s).unwrap();
                    let json = serde_json::to_string(&value).unwrap();

                    //let ident = syn::Ident::from(json);

                    Some(quote!(.body(#json)))
                }
            }
        }
    }
}
