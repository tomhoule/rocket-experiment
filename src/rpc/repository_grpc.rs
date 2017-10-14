// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_ETHICS_REPOSITORY_GET_SCHEMA: ::grpcio::Method<super::repository::GetSchemaParams, super::repository::EthicsSchema> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/EthicsRepository/GetSchema",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ETHICS_REPOSITORY_GET_EDITIONS: ::grpcio::Method<super::repository::GetEditionsParams, super::repository::Editions> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/EthicsRepository/GetEditions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ETHICS_REPOSITORY_CREATE_EDITION: ::grpcio::Method<super::repository::Edition, super::repository::Edition> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/EthicsRepository/CreateEdition",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct EthicsRepositoryClient {
    client: ::grpcio::Client,
}

impl EthicsRepositoryClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        EthicsRepositoryClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_schema_opt(&self, req: super::repository::GetSchemaParams, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::repository::EthicsSchema> {
        self.client.unary_call(&METHOD_ETHICS_REPOSITORY_GET_SCHEMA, req, opt)
    }

    pub fn get_schema(&self, req: super::repository::GetSchemaParams) -> ::grpcio::Result<super::repository::EthicsSchema> {
        self.get_schema_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_schema_async_opt(&self, req: super::repository::GetSchemaParams, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::repository::EthicsSchema> {
        self.client.unary_call_async(&METHOD_ETHICS_REPOSITORY_GET_SCHEMA, req, opt)
    }

    pub fn get_schema_async(&self, req: super::repository::GetSchemaParams) -> ::grpcio::ClientUnaryReceiver<super::repository::EthicsSchema> {
        self.get_schema_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_editions_opt(&self, req: super::repository::GetEditionsParams, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::repository::Editions> {
        self.client.unary_call(&METHOD_ETHICS_REPOSITORY_GET_EDITIONS, req, opt)
    }

    pub fn get_editions(&self, req: super::repository::GetEditionsParams) -> ::grpcio::Result<super::repository::Editions> {
        self.get_editions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_editions_async_opt(&self, req: super::repository::GetEditionsParams, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::repository::Editions> {
        self.client.unary_call_async(&METHOD_ETHICS_REPOSITORY_GET_EDITIONS, req, opt)
    }

    pub fn get_editions_async(&self, req: super::repository::GetEditionsParams) -> ::grpcio::ClientUnaryReceiver<super::repository::Editions> {
        self.get_editions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_edition_opt(&self, req: super::repository::Edition, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::repository::Edition> {
        self.client.unary_call(&METHOD_ETHICS_REPOSITORY_CREATE_EDITION, req, opt)
    }

    pub fn create_edition(&self, req: super::repository::Edition) -> ::grpcio::Result<super::repository::Edition> {
        self.create_edition_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_edition_async_opt(&self, req: super::repository::Edition, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::repository::Edition> {
        self.client.unary_call_async(&METHOD_ETHICS_REPOSITORY_CREATE_EDITION, req, opt)
    }

    pub fn create_edition_async(&self, req: super::repository::Edition) -> ::grpcio::ClientUnaryReceiver<super::repository::Edition> {
        self.create_edition_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait EthicsRepository {
    fn get_schema(&self, ctx: ::grpcio::RpcContext, req: super::repository::GetSchemaParams, sink: ::grpcio::UnarySink<super::repository::EthicsSchema>);
    fn get_editions(&self, ctx: ::grpcio::RpcContext, req: super::repository::GetEditionsParams, sink: ::grpcio::UnarySink<super::repository::Editions>);
    fn create_edition(&self, ctx: ::grpcio::RpcContext, req: super::repository::Edition, sink: ::grpcio::UnarySink<super::repository::Edition>);
}

pub fn create_ethics_repository<S: EthicsRepository + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ETHICS_REPOSITORY_GET_SCHEMA, move |ctx, req, resp| {
        instance.get_schema(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ETHICS_REPOSITORY_GET_EDITIONS, move |ctx, req, resp| {
        instance.get_editions(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ETHICS_REPOSITORY_CREATE_EDITION, move |ctx, req, resp| {
        instance.create_edition(ctx, req, resp)
    });
    builder.build()
}
