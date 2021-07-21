use std::iter::FromIterator;
use syn::Ident;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, Generics};
use syn::{DeriveInput, Type};

type IdentRaw=syn::Ident;
type IdentT=syn::Ident;

#[proc_macro_derive(FromMap)]
pub fn from_map(input:TokenStream)->TokenStream{
    let ast=&syn::parse(input).unwrap();
    handle(ast)
}

fn handle(ast:&DeriveInput)->TokenStream{
    let name=&ast.ident;

    let generics=&ast.generics;
    let (g_ident,g_where)=load_generics(generics);

    let data_raw=&ast.data;
    let data=load_data(data_raw).unwrap();
    let data_c=data.clone();
    let get_data=data_c.iter()
    .map(|f|{
        let name = &f.0;
        let temp_name=format_ident!("__temp__{}__",name);
        let map_name=&f.1;
        let ty = &f.2;
        let (t,b)=load_type(ty);
        quote! {
            let #temp_name = map.get(stringify!(#map_name))?.into_chain();
            let #name  :#t = <#b as msg_chain::FromChainMeta>::from_chain(Some(&#temp_name))?;
        }
    });

    let create_names=data.iter()
    .map(|f|f.0.clone());

    let gen=quote! {
        impl #g_ident FromMap  for #name #g_ident #g_where {
            fn from_map(map:&std::collections::HashMap<String,Value>)->Option<Self>{
                #( #get_data )*

                Some(#name{
                    #(#create_names),*
                })
            }
        }
    };


    gen.into()
}

fn load_generics(g: &Generics) -> (quote::__private::TokenStream, quote::__private::TokenStream) {
    //can set to where
    let type_params = g.type_params();
    let lifetimes = g.lifetimes();
    //can not set to where
    let const_params = g
        .const_params()
        .into_iter()
        .map(|f| quote! {#f})
        .collect::<Vec<_>>();

    let where_clause = g.where_clause.clone();
    let mut used = Vec::new();
    let mut limits = Vec::new();
    // load life time
    for lifetime in lifetimes.into_iter() {
        let life = &lifetime.lifetime;
        let has_limit = &lifetime.bounds.len() > &0;
        used.push(quote! {#life});

        let t = if has_limit {
            quote! {
                #lifetime
            }
        } else {
            quote! {}
        };
        limits.push(t);
    }
    //load type params
    for type_param in type_params.into_iter() {
        let base = &type_param.ident;
        let has_limits = type_param.bounds.len() > 0;
        used.push(quote! {#base});
        let t = if has_limits {
            quote! {
                #type_param
            }
        } else {
            quote! {}
        };
        limits.push(t);
    }

    let mut where_limit = Vec::new();
    // genreate where
    if let Some(where_clause) = where_clause {
        for wh in where_clause.predicates {
            where_limit.push(quote! {
                #wh
            })
        }
    }

    for limit in limits {
        where_limit.push(limit);
    }
    let where_limit = where_limit.iter();
    let sub_where = if where_limit.len() > 0 {
        quote! {
            where
            #( #where_limit),*
        }
    } else {
        quote! {}
    };

    let g_useds = used.iter();
    // use gereric
    let g = quote! {
        <
            # ( #g_useds ),*
            #( #const_params),*
        >
    };

    (g, sub_where)
}


fn load_data(data: &Data) -> Option<Vec<(IdentRaw,IdentT, Type)>> {
    if let Data::Struct(st) = data {
        let fields = &st.fields;
        match fields {
            syn::Fields::Named(ns) => {
                let fields = &ns.named;
                let res = fields
                    .into_iter()
                    .map(|f| (&f.ident, &f.ty))
                    .filter(|predicate| if let None = predicate.0 { false } else { true })
                    .map(|f| (f.0.clone().unwrap(),transfrom_name(f.0.clone().unwrap().to_string()), f.1.clone()))
                    
                    .collect::<Vec<_>>();
                Some(res)
            }
            syn::Fields::Unnamed(_) => (None),
            syn::Fields::Unit => (None),
        }
    } else {
        None
    }
}


fn load_type(ty:&Type)->(quote::__private::TokenStream,quote::__private::TokenStream){
    let ty_def=quote! {#ty};
    if let Type::Path(p) = ty {
        let path=&p.path;
        let seg=&path.segments;
        let mut paths=Vec::new();

        for seg in seg.into_iter(){
            let ident=&seg.ident;
            paths.push(ident);
        }

        let paths=paths.iter();
        let base=quote! {#(#paths)::*};
        (ty_def,base)
    }else{
        (
            quote! {#ty},
            quote! {#ty}
        )
    }   
}

fn transfrom_name(name:String)->Ident{
    name.split("_")
    .into_iter()
    .map(|f|f.chars())
    .map(|mut f|{
        let mut s=String::new();
        let first=f.next();
        let left_str=String::from_iter(f);
        if let Some(ch)=first{
            s.push_str(&ch.to_uppercase().to_string());
            s.push_str(&left_str);
        }
        s
    })
    .reduce(|mut f,b|{f.push_str(&b);f})

    .and_then(|f|{
        let mut ch_iter=f.chars();
        let first=ch_iter.next().unwrap();
        let left=String::from_iter(ch_iter);
        let mut t=String::from(first.to_lowercase().to_string());
        t.push_str(&left);

        Some(t)
    })
    .and_then(|f|Some(format_ident!("{}",f))).unwrap()
}
