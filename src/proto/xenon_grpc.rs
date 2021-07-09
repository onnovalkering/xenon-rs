// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]
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

const METHOD_FILE_SYSTEM_SERVICE_GET_ADAPTOR_DESCRIPTIONS: ::grpcio::Method<
    super::xenon::Empty,
    super::xenon::FileSystemAdaptorDescriptions,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/xenon.FileSystemService/getAdaptorDescriptions",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_FILE_SYSTEM_SERVICE_GET_ADAPTOR_NAMES: ::grpcio::Method<super::xenon::Empty, super::xenon::AdaptorNames> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.FileSystemService/getAdaptorNames",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_FILE_SYSTEM_SERVICE_GET_ADAPTOR_DESCRIPTION: ::grpcio::Method<
    super::xenon::AdaptorName,
    super::xenon::FileSystemAdaptorDescription,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/xenon.FileSystemService/getAdaptorDescription",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_FILE_SYSTEM_SERVICE_CREATE: ::grpcio::Method<
    super::xenon::CreateFileSystemRequest,
    super::xenon::FileSystem,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/xenon.FileSystemService/create",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_FILE_SYSTEM_SERVICE_GET_ADAPTOR_NAME: ::grpcio::Method<
    super::xenon::FileSystem,
    super::xenon::AdaptorName,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/xenon.FileSystemService/getAdaptorName",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_FILE_SYSTEM_SERVICE_GET_LOCATION: ::grpcio::Method<super::xenon::FileSystem, super::xenon::Location> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.FileSystemService/getLocation",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_FILE_SYSTEM_SERVICE_GET_CREDENTIAL: ::grpcio::Method<
    super::xenon::FileSystem,
    super::xenon::GetCredentialResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/xenon.FileSystemService/getCredential",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_FILE_SYSTEM_SERVICE_GET_PROPERTIES: ::grpcio::Method<super::xenon::FileSystem, super::xenon::Properties> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.FileSystemService/getProperties",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_FILE_SYSTEM_SERVICE_CREATE_DIRECTORIES: ::grpcio::Method<super::xenon::PathRequest, super::xenon::Empty> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.FileSystemService/createDirectories",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_FILE_SYSTEM_SERVICE_CREATE_DIRECTORY: ::grpcio::Method<super::xenon::PathRequest, super::xenon::Empty> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.FileSystemService/createDirectory",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_FILE_SYSTEM_SERVICE_CREATE_FILE: ::grpcio::Method<super::xenon::PathRequest, super::xenon::Empty> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.FileSystemService/createFile",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_FILE_SYSTEM_SERVICE_CREATE_SYMBOLIC_LINK: ::grpcio::Method<
    super::xenon::CreateSymbolicLinkRequest,
    super::xenon::Empty,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/xenon.FileSystemService/createSymbolicLink",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_FILE_SYSTEM_SERVICE_COPY: ::grpcio::Method<super::xenon::CopyRequest, super::xenon::CopyOperation> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.FileSystemService/copy",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_FILE_SYSTEM_SERVICE_CANCEL: ::grpcio::Method<
    super::xenon::CopyOperationRequest,
    super::xenon::CopyStatus,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/xenon.FileSystemService/cancel",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_FILE_SYSTEM_SERVICE_GET_STATUS: ::grpcio::Method<
    super::xenon::CopyOperationRequest,
    super::xenon::CopyStatus,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/xenon.FileSystemService/getStatus",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_FILE_SYSTEM_SERVICE_RENAME: ::grpcio::Method<super::xenon::RenameRequest, super::xenon::Empty> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.FileSystemService/rename",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_FILE_SYSTEM_SERVICE_DELETE: ::grpcio::Method<super::xenon::DeleteRequest, super::xenon::Empty> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.FileSystemService/delete",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_FILE_SYSTEM_SERVICE_EXISTS: ::grpcio::Method<super::xenon::PathRequest, super::xenon::Is> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.FileSystemService/exists",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_FILE_SYSTEM_SERVICE_READ_FROM_FILE: ::grpcio::Method<
    super::xenon::PathRequest,
    super::xenon::ReadFromFileResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/xenon.FileSystemService/readFromFile",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_FILE_SYSTEM_SERVICE_WRITE_TO_FILE: ::grpcio::Method<
    super::xenon::WriteToFileRequest,
    super::xenon::Empty,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/xenon.FileSystemService/writeToFile",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_FILE_SYSTEM_SERVICE_APPEND_TO_FILE: ::grpcio::Method<
    super::xenon::AppendToFileRequest,
    super::xenon::Empty,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/xenon.FileSystemService/appendToFile",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_FILE_SYSTEM_SERVICE_LIST: ::grpcio::Method<super::xenon::ListRequest, super::xenon::PathAttributes> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::ServerStreaming,
        name: "/xenon.FileSystemService/list",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_FILE_SYSTEM_SERVICE_GET_ATTRIBUTES: ::grpcio::Method<
    super::xenon::PathRequest,
    super::xenon::PathAttributes,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/xenon.FileSystemService/getAttributes",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_FILE_SYSTEM_SERVICE_GET_WORKING_DIRECTORY: ::grpcio::Method<super::xenon::FileSystem, super::xenon::Path> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.FileSystemService/getWorkingDirectory",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_FILE_SYSTEM_SERVICE_SET_WORKING_DIRECTORY: ::grpcio::Method<
    super::xenon::PathRequest,
    super::xenon::Empty,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/xenon.FileSystemService/setWorkingDirectory",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_FILE_SYSTEM_SERVICE_SET_POSIX_FILE_PERMISSIONS: ::grpcio::Method<
    super::xenon::SetPosixFilePermissionsRequest,
    super::xenon::Empty,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/xenon.FileSystemService/setPosixFilePermissions",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_FILE_SYSTEM_SERVICE_READ_SYMBOLIC_LINK: ::grpcio::Method<super::xenon::PathRequest, super::xenon::Path> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.FileSystemService/readSymbolicLink",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_FILE_SYSTEM_SERVICE_GET_PATH_SEPARATOR: ::grpcio::Method<
    super::xenon::FileSystem,
    super::xenon::GetPathSeparatorResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/xenon.FileSystemService/getPathSeparator",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_FILE_SYSTEM_SERVICE_IS_OPEN: ::grpcio::Method<super::xenon::FileSystem, super::xenon::Is> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.FileSystemService/isOpen",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_FILE_SYSTEM_SERVICE_CLOSE: ::grpcio::Method<super::xenon::FileSystem, super::xenon::Empty> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.FileSystemService/close",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_FILE_SYSTEM_SERVICE_WAIT_UNTIL_DONE: ::grpcio::Method<
    super::xenon::WaitUntilDoneRequest,
    super::xenon::CopyStatus,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/xenon.FileSystemService/waitUntilDone",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_FILE_SYSTEM_SERVICE_LOCAL_FILE_SYSTEMS: ::grpcio::Method<super::xenon::Empty, super::xenon::FileSystems> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.FileSystemService/localFileSystems",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_FILE_SYSTEM_SERVICE_LIST_FILE_SYSTEMS: ::grpcio::Method<super::xenon::Empty, super::xenon::FileSystems> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.FileSystemService/listFileSystems",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

#[derive(Clone)]
pub struct FileSystemServiceClient {
    client: ::grpcio::Client,
}

impl FileSystemServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        FileSystemServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_adaptor_descriptions_opt(
        &self,
        req: &super::xenon::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::FileSystemAdaptorDescriptions> {
        self.client
            .unary_call(&METHOD_FILE_SYSTEM_SERVICE_GET_ADAPTOR_DESCRIPTIONS, req, opt)
    }

    pub fn get_adaptor_descriptions(
        &self,
        req: &super::xenon::Empty,
    ) -> ::grpcio::Result<super::xenon::FileSystemAdaptorDescriptions> {
        self.get_adaptor_descriptions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_adaptor_descriptions_async_opt(
        &self,
        req: &super::xenon::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::FileSystemAdaptorDescriptions>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_GET_ADAPTOR_DESCRIPTIONS, req, opt)
    }

    pub fn get_adaptor_descriptions_async(
        &self,
        req: &super::xenon::Empty,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::FileSystemAdaptorDescriptions>> {
        self.get_adaptor_descriptions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_adaptor_names_opt(
        &self,
        req: &super::xenon::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::AdaptorNames> {
        self.client
            .unary_call(&METHOD_FILE_SYSTEM_SERVICE_GET_ADAPTOR_NAMES, req, opt)
    }

    pub fn get_adaptor_names(
        &self,
        req: &super::xenon::Empty,
    ) -> ::grpcio::Result<super::xenon::AdaptorNames> {
        self.get_adaptor_names_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_adaptor_names_async_opt(
        &self,
        req: &super::xenon::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::AdaptorNames>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_GET_ADAPTOR_NAMES, req, opt)
    }

    pub fn get_adaptor_names_async(
        &self,
        req: &super::xenon::Empty,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::AdaptorNames>> {
        self.get_adaptor_names_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_adaptor_description_opt(
        &self,
        req: &super::xenon::AdaptorName,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::FileSystemAdaptorDescription> {
        self.client
            .unary_call(&METHOD_FILE_SYSTEM_SERVICE_GET_ADAPTOR_DESCRIPTION, req, opt)
    }

    pub fn get_adaptor_description(
        &self,
        req: &super::xenon::AdaptorName,
    ) -> ::grpcio::Result<super::xenon::FileSystemAdaptorDescription> {
        self.get_adaptor_description_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_adaptor_description_async_opt(
        &self,
        req: &super::xenon::AdaptorName,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::FileSystemAdaptorDescription>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_GET_ADAPTOR_DESCRIPTION, req, opt)
    }

    pub fn get_adaptor_description_async(
        &self,
        req: &super::xenon::AdaptorName,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::FileSystemAdaptorDescription>> {
        self.get_adaptor_description_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_opt(
        &self,
        req: &super::xenon::CreateFileSystemRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::FileSystem> {
        self.client.unary_call(&METHOD_FILE_SYSTEM_SERVICE_CREATE, req, opt)
    }

    pub fn create(
        &self,
        req: &super::xenon::CreateFileSystemRequest,
    ) -> ::grpcio::Result<super::xenon::FileSystem> {
        self.create_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_async_opt(
        &self,
        req: &super::xenon::CreateFileSystemRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::FileSystem>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_CREATE, req, opt)
    }

    pub fn create_async(
        &self,
        req: &super::xenon::CreateFileSystemRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::FileSystem>> {
        self.create_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_adaptor_name_opt(
        &self,
        req: &super::xenon::FileSystem,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::AdaptorName> {
        self.client
            .unary_call(&METHOD_FILE_SYSTEM_SERVICE_GET_ADAPTOR_NAME, req, opt)
    }

    pub fn get_adaptor_name(
        &self,
        req: &super::xenon::FileSystem,
    ) -> ::grpcio::Result<super::xenon::AdaptorName> {
        self.get_adaptor_name_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_adaptor_name_async_opt(
        &self,
        req: &super::xenon::FileSystem,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::AdaptorName>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_GET_ADAPTOR_NAME, req, opt)
    }

    pub fn get_adaptor_name_async(
        &self,
        req: &super::xenon::FileSystem,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::AdaptorName>> {
        self.get_adaptor_name_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_location_opt(
        &self,
        req: &super::xenon::FileSystem,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Location> {
        self.client
            .unary_call(&METHOD_FILE_SYSTEM_SERVICE_GET_LOCATION, req, opt)
    }

    pub fn get_location(
        &self,
        req: &super::xenon::FileSystem,
    ) -> ::grpcio::Result<super::xenon::Location> {
        self.get_location_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_location_async_opt(
        &self,
        req: &super::xenon::FileSystem,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Location>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_GET_LOCATION, req, opt)
    }

    pub fn get_location_async(
        &self,
        req: &super::xenon::FileSystem,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Location>> {
        self.get_location_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_credential_opt(
        &self,
        req: &super::xenon::FileSystem,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::GetCredentialResponse> {
        self.client
            .unary_call(&METHOD_FILE_SYSTEM_SERVICE_GET_CREDENTIAL, req, opt)
    }

    pub fn get_credential(
        &self,
        req: &super::xenon::FileSystem,
    ) -> ::grpcio::Result<super::xenon::GetCredentialResponse> {
        self.get_credential_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_credential_async_opt(
        &self,
        req: &super::xenon::FileSystem,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::GetCredentialResponse>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_GET_CREDENTIAL, req, opt)
    }

    pub fn get_credential_async(
        &self,
        req: &super::xenon::FileSystem,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::GetCredentialResponse>> {
        self.get_credential_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_properties_opt(
        &self,
        req: &super::xenon::FileSystem,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Properties> {
        self.client
            .unary_call(&METHOD_FILE_SYSTEM_SERVICE_GET_PROPERTIES, req, opt)
    }

    pub fn get_properties(
        &self,
        req: &super::xenon::FileSystem,
    ) -> ::grpcio::Result<super::xenon::Properties> {
        self.get_properties_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_properties_async_opt(
        &self,
        req: &super::xenon::FileSystem,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Properties>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_GET_PROPERTIES, req, opt)
    }

    pub fn get_properties_async(
        &self,
        req: &super::xenon::FileSystem,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Properties>> {
        self.get_properties_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_directories_opt(
        &self,
        req: &super::xenon::PathRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Empty> {
        self.client
            .unary_call(&METHOD_FILE_SYSTEM_SERVICE_CREATE_DIRECTORIES, req, opt)
    }

    pub fn create_directories(
        &self,
        req: &super::xenon::PathRequest,
    ) -> ::grpcio::Result<super::xenon::Empty> {
        self.create_directories_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_directories_async_opt(
        &self,
        req: &super::xenon::PathRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Empty>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_CREATE_DIRECTORIES, req, opt)
    }

    pub fn create_directories_async(
        &self,
        req: &super::xenon::PathRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Empty>> {
        self.create_directories_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_directory_opt(
        &self,
        req: &super::xenon::PathRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Empty> {
        self.client
            .unary_call(&METHOD_FILE_SYSTEM_SERVICE_CREATE_DIRECTORY, req, opt)
    }

    pub fn create_directory(
        &self,
        req: &super::xenon::PathRequest,
    ) -> ::grpcio::Result<super::xenon::Empty> {
        self.create_directory_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_directory_async_opt(
        &self,
        req: &super::xenon::PathRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Empty>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_CREATE_DIRECTORY, req, opt)
    }

    pub fn create_directory_async(
        &self,
        req: &super::xenon::PathRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Empty>> {
        self.create_directory_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_file_opt(
        &self,
        req: &super::xenon::PathRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Empty> {
        self.client
            .unary_call(&METHOD_FILE_SYSTEM_SERVICE_CREATE_FILE, req, opt)
    }

    pub fn create_file(
        &self,
        req: &super::xenon::PathRequest,
    ) -> ::grpcio::Result<super::xenon::Empty> {
        self.create_file_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_file_async_opt(
        &self,
        req: &super::xenon::PathRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Empty>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_CREATE_FILE, req, opt)
    }

    pub fn create_file_async(
        &self,
        req: &super::xenon::PathRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Empty>> {
        self.create_file_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_symbolic_link_opt(
        &self,
        req: &super::xenon::CreateSymbolicLinkRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Empty> {
        self.client
            .unary_call(&METHOD_FILE_SYSTEM_SERVICE_CREATE_SYMBOLIC_LINK, req, opt)
    }

    pub fn create_symbolic_link(
        &self,
        req: &super::xenon::CreateSymbolicLinkRequest,
    ) -> ::grpcio::Result<super::xenon::Empty> {
        self.create_symbolic_link_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_symbolic_link_async_opt(
        &self,
        req: &super::xenon::CreateSymbolicLinkRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Empty>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_CREATE_SYMBOLIC_LINK, req, opt)
    }

    pub fn create_symbolic_link_async(
        &self,
        req: &super::xenon::CreateSymbolicLinkRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Empty>> {
        self.create_symbolic_link_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn copy_opt(
        &self,
        req: &super::xenon::CopyRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::CopyOperation> {
        self.client.unary_call(&METHOD_FILE_SYSTEM_SERVICE_COPY, req, opt)
    }

    pub fn copy(
        &self,
        req: &super::xenon::CopyRequest,
    ) -> ::grpcio::Result<super::xenon::CopyOperation> {
        self.copy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn copy_async_opt(
        &self,
        req: &super::xenon::CopyRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::CopyOperation>> {
        self.client.unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_COPY, req, opt)
    }

    pub fn copy_async(
        &self,
        req: &super::xenon::CopyRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::CopyOperation>> {
        self.copy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_opt(
        &self,
        req: &super::xenon::CopyOperationRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::CopyStatus> {
        self.client.unary_call(&METHOD_FILE_SYSTEM_SERVICE_CANCEL, req, opt)
    }

    pub fn cancel(
        &self,
        req: &super::xenon::CopyOperationRequest,
    ) -> ::grpcio::Result<super::xenon::CopyStatus> {
        self.cancel_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_async_opt(
        &self,
        req: &super::xenon::CopyOperationRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::CopyStatus>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_CANCEL, req, opt)
    }

    pub fn cancel_async(
        &self,
        req: &super::xenon::CopyOperationRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::CopyStatus>> {
        self.cancel_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_status_opt(
        &self,
        req: &super::xenon::CopyOperationRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::CopyStatus> {
        self.client.unary_call(&METHOD_FILE_SYSTEM_SERVICE_GET_STATUS, req, opt)
    }

    pub fn get_status(
        &self,
        req: &super::xenon::CopyOperationRequest,
    ) -> ::grpcio::Result<super::xenon::CopyStatus> {
        self.get_status_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_status_async_opt(
        &self,
        req: &super::xenon::CopyOperationRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::CopyStatus>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_GET_STATUS, req, opt)
    }

    pub fn get_status_async(
        &self,
        req: &super::xenon::CopyOperationRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::CopyStatus>> {
        self.get_status_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn rename_opt(
        &self,
        req: &super::xenon::RenameRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Empty> {
        self.client.unary_call(&METHOD_FILE_SYSTEM_SERVICE_RENAME, req, opt)
    }

    pub fn rename(
        &self,
        req: &super::xenon::RenameRequest,
    ) -> ::grpcio::Result<super::xenon::Empty> {
        self.rename_opt(req, ::grpcio::CallOption::default())
    }

    pub fn rename_async_opt(
        &self,
        req: &super::xenon::RenameRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Empty>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_RENAME, req, opt)
    }

    pub fn rename_async(
        &self,
        req: &super::xenon::RenameRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Empty>> {
        self.rename_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_opt(
        &self,
        req: &super::xenon::DeleteRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Empty> {
        self.client.unary_call(&METHOD_FILE_SYSTEM_SERVICE_DELETE, req, opt)
    }

    pub fn delete(
        &self,
        req: &super::xenon::DeleteRequest,
    ) -> ::grpcio::Result<super::xenon::Empty> {
        self.delete_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_async_opt(
        &self,
        req: &super::xenon::DeleteRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Empty>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_DELETE, req, opt)
    }

    pub fn delete_async(
        &self,
        req: &super::xenon::DeleteRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Empty>> {
        self.delete_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn exists_opt(
        &self,
        req: &super::xenon::PathRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Is> {
        self.client.unary_call(&METHOD_FILE_SYSTEM_SERVICE_EXISTS, req, opt)
    }

    pub fn exists(
        &self,
        req: &super::xenon::PathRequest,
    ) -> ::grpcio::Result<super::xenon::Is> {
        self.exists_opt(req, ::grpcio::CallOption::default())
    }

    pub fn exists_async_opt(
        &self,
        req: &super::xenon::PathRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Is>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_EXISTS, req, opt)
    }

    pub fn exists_async(
        &self,
        req: &super::xenon::PathRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Is>> {
        self.exists_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn read_from_file_opt(
        &self,
        req: &super::xenon::PathRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::xenon::ReadFromFileResponse>> {
        self.client
            .server_streaming(&METHOD_FILE_SYSTEM_SERVICE_READ_FROM_FILE, req, opt)
    }

    pub fn read_from_file(
        &self,
        req: &super::xenon::PathRequest,
    ) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::xenon::ReadFromFileResponse>> {
        self.read_from_file_opt(req, ::grpcio::CallOption::default())
    }

    pub fn write_to_file_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientCStreamSender<super::xenon::WriteToFileRequest>,
        ::grpcio::ClientCStreamReceiver<super::xenon::Empty>,
    )> {
        self.client
            .client_streaming(&METHOD_FILE_SYSTEM_SERVICE_WRITE_TO_FILE, opt)
    }

    pub fn write_to_file(
        &self
    ) -> ::grpcio::Result<(
        ::grpcio::ClientCStreamSender<super::xenon::WriteToFileRequest>,
        ::grpcio::ClientCStreamReceiver<super::xenon::Empty>,
    )> {
        self.write_to_file_opt(::grpcio::CallOption::default())
    }

    pub fn append_to_file_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientCStreamSender<super::xenon::AppendToFileRequest>,
        ::grpcio::ClientCStreamReceiver<super::xenon::Empty>,
    )> {
        self.client
            .client_streaming(&METHOD_FILE_SYSTEM_SERVICE_APPEND_TO_FILE, opt)
    }

    pub fn append_to_file(
        &self
    ) -> ::grpcio::Result<(
        ::grpcio::ClientCStreamSender<super::xenon::AppendToFileRequest>,
        ::grpcio::ClientCStreamReceiver<super::xenon::Empty>,
    )> {
        self.append_to_file_opt(::grpcio::CallOption::default())
    }

    pub fn list_opt(
        &self,
        req: &super::xenon::ListRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::xenon::PathAttributes>> {
        self.client.server_streaming(&METHOD_FILE_SYSTEM_SERVICE_LIST, req, opt)
    }

    pub fn list(
        &self,
        req: &super::xenon::ListRequest,
    ) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::xenon::PathAttributes>> {
        self.list_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_attributes_opt(
        &self,
        req: &super::xenon::PathRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::PathAttributes> {
        self.client
            .unary_call(&METHOD_FILE_SYSTEM_SERVICE_GET_ATTRIBUTES, req, opt)
    }

    pub fn get_attributes(
        &self,
        req: &super::xenon::PathRequest,
    ) -> ::grpcio::Result<super::xenon::PathAttributes> {
        self.get_attributes_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_attributes_async_opt(
        &self,
        req: &super::xenon::PathRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::PathAttributes>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_GET_ATTRIBUTES, req, opt)
    }

    pub fn get_attributes_async(
        &self,
        req: &super::xenon::PathRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::PathAttributes>> {
        self.get_attributes_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_working_directory_opt(
        &self,
        req: &super::xenon::FileSystem,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Path> {
        self.client
            .unary_call(&METHOD_FILE_SYSTEM_SERVICE_GET_WORKING_DIRECTORY, req, opt)
    }

    pub fn get_working_directory(
        &self,
        req: &super::xenon::FileSystem,
    ) -> ::grpcio::Result<super::xenon::Path> {
        self.get_working_directory_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_working_directory_async_opt(
        &self,
        req: &super::xenon::FileSystem,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Path>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_GET_WORKING_DIRECTORY, req, opt)
    }

    pub fn get_working_directory_async(
        &self,
        req: &super::xenon::FileSystem,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Path>> {
        self.get_working_directory_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_working_directory_opt(
        &self,
        req: &super::xenon::PathRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Empty> {
        self.client
            .unary_call(&METHOD_FILE_SYSTEM_SERVICE_SET_WORKING_DIRECTORY, req, opt)
    }

    pub fn set_working_directory(
        &self,
        req: &super::xenon::PathRequest,
    ) -> ::grpcio::Result<super::xenon::Empty> {
        self.set_working_directory_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_working_directory_async_opt(
        &self,
        req: &super::xenon::PathRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Empty>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_SET_WORKING_DIRECTORY, req, opt)
    }

    pub fn set_working_directory_async(
        &self,
        req: &super::xenon::PathRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Empty>> {
        self.set_working_directory_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_posix_file_permissions_opt(
        &self,
        req: &super::xenon::SetPosixFilePermissionsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Empty> {
        self.client
            .unary_call(&METHOD_FILE_SYSTEM_SERVICE_SET_POSIX_FILE_PERMISSIONS, req, opt)
    }

    pub fn set_posix_file_permissions(
        &self,
        req: &super::xenon::SetPosixFilePermissionsRequest,
    ) -> ::grpcio::Result<super::xenon::Empty> {
        self.set_posix_file_permissions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_posix_file_permissions_async_opt(
        &self,
        req: &super::xenon::SetPosixFilePermissionsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Empty>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_SET_POSIX_FILE_PERMISSIONS, req, opt)
    }

    pub fn set_posix_file_permissions_async(
        &self,
        req: &super::xenon::SetPosixFilePermissionsRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Empty>> {
        self.set_posix_file_permissions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn read_symbolic_link_opt(
        &self,
        req: &super::xenon::PathRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Path> {
        self.client
            .unary_call(&METHOD_FILE_SYSTEM_SERVICE_READ_SYMBOLIC_LINK, req, opt)
    }

    pub fn read_symbolic_link(
        &self,
        req: &super::xenon::PathRequest,
    ) -> ::grpcio::Result<super::xenon::Path> {
        self.read_symbolic_link_opt(req, ::grpcio::CallOption::default())
    }

    pub fn read_symbolic_link_async_opt(
        &self,
        req: &super::xenon::PathRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Path>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_READ_SYMBOLIC_LINK, req, opt)
    }

    pub fn read_symbolic_link_async(
        &self,
        req: &super::xenon::PathRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Path>> {
        self.read_symbolic_link_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_path_separator_opt(
        &self,
        req: &super::xenon::FileSystem,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::GetPathSeparatorResponse> {
        self.client
            .unary_call(&METHOD_FILE_SYSTEM_SERVICE_GET_PATH_SEPARATOR, req, opt)
    }

    pub fn get_path_separator(
        &self,
        req: &super::xenon::FileSystem,
    ) -> ::grpcio::Result<super::xenon::GetPathSeparatorResponse> {
        self.get_path_separator_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_path_separator_async_opt(
        &self,
        req: &super::xenon::FileSystem,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::GetPathSeparatorResponse>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_GET_PATH_SEPARATOR, req, opt)
    }

    pub fn get_path_separator_async(
        &self,
        req: &super::xenon::FileSystem,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::GetPathSeparatorResponse>> {
        self.get_path_separator_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn is_open_opt(
        &self,
        req: &super::xenon::FileSystem,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Is> {
        self.client.unary_call(&METHOD_FILE_SYSTEM_SERVICE_IS_OPEN, req, opt)
    }

    pub fn is_open(
        &self,
        req: &super::xenon::FileSystem,
    ) -> ::grpcio::Result<super::xenon::Is> {
        self.is_open_opt(req, ::grpcio::CallOption::default())
    }

    pub fn is_open_async_opt(
        &self,
        req: &super::xenon::FileSystem,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Is>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_IS_OPEN, req, opt)
    }

    pub fn is_open_async(
        &self,
        req: &super::xenon::FileSystem,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Is>> {
        self.is_open_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn close_opt(
        &self,
        req: &super::xenon::FileSystem,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Empty> {
        self.client.unary_call(&METHOD_FILE_SYSTEM_SERVICE_CLOSE, req, opt)
    }

    pub fn close(
        &self,
        req: &super::xenon::FileSystem,
    ) -> ::grpcio::Result<super::xenon::Empty> {
        self.close_opt(req, ::grpcio::CallOption::default())
    }

    pub fn close_async_opt(
        &self,
        req: &super::xenon::FileSystem,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Empty>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_CLOSE, req, opt)
    }

    pub fn close_async(
        &self,
        req: &super::xenon::FileSystem,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Empty>> {
        self.close_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn wait_until_done_opt(
        &self,
        req: &super::xenon::WaitUntilDoneRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::CopyStatus> {
        self.client
            .unary_call(&METHOD_FILE_SYSTEM_SERVICE_WAIT_UNTIL_DONE, req, opt)
    }

    pub fn wait_until_done(
        &self,
        req: &super::xenon::WaitUntilDoneRequest,
    ) -> ::grpcio::Result<super::xenon::CopyStatus> {
        self.wait_until_done_opt(req, ::grpcio::CallOption::default())
    }

    pub fn wait_until_done_async_opt(
        &self,
        req: &super::xenon::WaitUntilDoneRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::CopyStatus>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_WAIT_UNTIL_DONE, req, opt)
    }

    pub fn wait_until_done_async(
        &self,
        req: &super::xenon::WaitUntilDoneRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::CopyStatus>> {
        self.wait_until_done_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn local_file_systems_opt(
        &self,
        req: &super::xenon::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::FileSystems> {
        self.client
            .unary_call(&METHOD_FILE_SYSTEM_SERVICE_LOCAL_FILE_SYSTEMS, req, opt)
    }

    pub fn local_file_systems(
        &self,
        req: &super::xenon::Empty,
    ) -> ::grpcio::Result<super::xenon::FileSystems> {
        self.local_file_systems_opt(req, ::grpcio::CallOption::default())
    }

    pub fn local_file_systems_async_opt(
        &self,
        req: &super::xenon::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::FileSystems>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_LOCAL_FILE_SYSTEMS, req, opt)
    }

    pub fn local_file_systems_async(
        &self,
        req: &super::xenon::Empty,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::FileSystems>> {
        self.local_file_systems_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_file_systems_opt(
        &self,
        req: &super::xenon::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::FileSystems> {
        self.client
            .unary_call(&METHOD_FILE_SYSTEM_SERVICE_LIST_FILE_SYSTEMS, req, opt)
    }

    pub fn list_file_systems(
        &self,
        req: &super::xenon::Empty,
    ) -> ::grpcio::Result<super::xenon::FileSystems> {
        self.list_file_systems_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_file_systems_async_opt(
        &self,
        req: &super::xenon::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::FileSystems>> {
        self.client
            .unary_call_async(&METHOD_FILE_SYSTEM_SERVICE_LIST_FILE_SYSTEMS, req, opt)
    }

    pub fn list_file_systems_async(
        &self,
        req: &super::xenon::Empty,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::FileSystems>> {
        self.list_file_systems_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(
        &self,
        f: F,
    ) where
        F: ::futures::Future<Output = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}

pub trait FileSystemService {
    fn get_adaptor_descriptions(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::Empty,
        sink: ::grpcio::UnarySink<super::xenon::FileSystemAdaptorDescriptions>,
    );
    fn get_adaptor_names(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::Empty,
        sink: ::grpcio::UnarySink<super::xenon::AdaptorNames>,
    );
    fn get_adaptor_description(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::AdaptorName,
        sink: ::grpcio::UnarySink<super::xenon::FileSystemAdaptorDescription>,
    );
    fn create(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::CreateFileSystemRequest,
        sink: ::grpcio::UnarySink<super::xenon::FileSystem>,
    );
    fn get_adaptor_name(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::FileSystem,
        sink: ::grpcio::UnarySink<super::xenon::AdaptorName>,
    );
    fn get_location(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::FileSystem,
        sink: ::grpcio::UnarySink<super::xenon::Location>,
    );
    fn get_credential(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::FileSystem,
        sink: ::grpcio::UnarySink<super::xenon::GetCredentialResponse>,
    );
    fn get_properties(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::FileSystem,
        sink: ::grpcio::UnarySink<super::xenon::Properties>,
    );
    fn create_directories(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::PathRequest,
        sink: ::grpcio::UnarySink<super::xenon::Empty>,
    );
    fn create_directory(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::PathRequest,
        sink: ::grpcio::UnarySink<super::xenon::Empty>,
    );
    fn create_file(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::PathRequest,
        sink: ::grpcio::UnarySink<super::xenon::Empty>,
    );
    fn create_symbolic_link(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::CreateSymbolicLinkRequest,
        sink: ::grpcio::UnarySink<super::xenon::Empty>,
    );
    fn copy(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::CopyRequest,
        sink: ::grpcio::UnarySink<super::xenon::CopyOperation>,
    );
    fn cancel(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::CopyOperationRequest,
        sink: ::grpcio::UnarySink<super::xenon::CopyStatus>,
    );
    fn get_status(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::CopyOperationRequest,
        sink: ::grpcio::UnarySink<super::xenon::CopyStatus>,
    );
    fn rename(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::RenameRequest,
        sink: ::grpcio::UnarySink<super::xenon::Empty>,
    );
    fn delete(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::DeleteRequest,
        sink: ::grpcio::UnarySink<super::xenon::Empty>,
    );
    fn exists(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::PathRequest,
        sink: ::grpcio::UnarySink<super::xenon::Is>,
    );
    fn read_from_file(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::PathRequest,
        sink: ::grpcio::ServerStreamingSink<super::xenon::ReadFromFileResponse>,
    );
    fn write_to_file(
        &mut self,
        ctx: ::grpcio::RpcContext,
        stream: ::grpcio::RequestStream<super::xenon::WriteToFileRequest>,
        sink: ::grpcio::ClientStreamingSink<super::xenon::Empty>,
    );
    fn append_to_file(
        &mut self,
        ctx: ::grpcio::RpcContext,
        stream: ::grpcio::RequestStream<super::xenon::AppendToFileRequest>,
        sink: ::grpcio::ClientStreamingSink<super::xenon::Empty>,
    );
    fn list(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::ListRequest,
        sink: ::grpcio::ServerStreamingSink<super::xenon::PathAttributes>,
    );
    fn get_attributes(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::PathRequest,
        sink: ::grpcio::UnarySink<super::xenon::PathAttributes>,
    );
    fn get_working_directory(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::FileSystem,
        sink: ::grpcio::UnarySink<super::xenon::Path>,
    );
    fn set_working_directory(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::PathRequest,
        sink: ::grpcio::UnarySink<super::xenon::Empty>,
    );
    fn set_posix_file_permissions(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::SetPosixFilePermissionsRequest,
        sink: ::grpcio::UnarySink<super::xenon::Empty>,
    );
    fn read_symbolic_link(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::PathRequest,
        sink: ::grpcio::UnarySink<super::xenon::Path>,
    );
    fn get_path_separator(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::FileSystem,
        sink: ::grpcio::UnarySink<super::xenon::GetPathSeparatorResponse>,
    );
    fn is_open(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::FileSystem,
        sink: ::grpcio::UnarySink<super::xenon::Is>,
    );
    fn close(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::FileSystem,
        sink: ::grpcio::UnarySink<super::xenon::Empty>,
    );
    fn wait_until_done(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::WaitUntilDoneRequest,
        sink: ::grpcio::UnarySink<super::xenon::CopyStatus>,
    );
    fn local_file_systems(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::Empty,
        sink: ::grpcio::UnarySink<super::xenon::FileSystems>,
    );
    fn list_file_systems(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::Empty,
        sink: ::grpcio::UnarySink<super::xenon::FileSystems>,
    );
}

pub fn create_file_system_service<S: FileSystemService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_FILE_SYSTEM_SERVICE_GET_ADAPTOR_DESCRIPTIONS,
        move |ctx, req, resp| instance.get_adaptor_descriptions(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SYSTEM_SERVICE_GET_ADAPTOR_NAMES, move |ctx, req, resp| {
        instance.get_adaptor_names(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_FILE_SYSTEM_SERVICE_GET_ADAPTOR_DESCRIPTION,
        move |ctx, req, resp| instance.get_adaptor_description(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SYSTEM_SERVICE_CREATE, move |ctx, req, resp| {
        instance.create(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SYSTEM_SERVICE_GET_ADAPTOR_NAME, move |ctx, req, resp| {
        instance.get_adaptor_name(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SYSTEM_SERVICE_GET_LOCATION, move |ctx, req, resp| {
        instance.get_location(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SYSTEM_SERVICE_GET_CREDENTIAL, move |ctx, req, resp| {
        instance.get_credential(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SYSTEM_SERVICE_GET_PROPERTIES, move |ctx, req, resp| {
        instance.get_properties(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SYSTEM_SERVICE_CREATE_DIRECTORIES, move |ctx, req, resp| {
        instance.create_directories(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SYSTEM_SERVICE_CREATE_DIRECTORY, move |ctx, req, resp| {
        instance.create_directory(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SYSTEM_SERVICE_CREATE_FILE, move |ctx, req, resp| {
        instance.create_file(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_FILE_SYSTEM_SERVICE_CREATE_SYMBOLIC_LINK,
        move |ctx, req, resp| instance.create_symbolic_link(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SYSTEM_SERVICE_COPY, move |ctx, req, resp| {
        instance.copy(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SYSTEM_SERVICE_CANCEL, move |ctx, req, resp| {
        instance.cancel(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SYSTEM_SERVICE_GET_STATUS, move |ctx, req, resp| {
        instance.get_status(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SYSTEM_SERVICE_RENAME, move |ctx, req, resp| {
        instance.rename(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SYSTEM_SERVICE_DELETE, move |ctx, req, resp| {
        instance.delete(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SYSTEM_SERVICE_EXISTS, move |ctx, req, resp| {
        instance.exists(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder
        .add_server_streaming_handler(&METHOD_FILE_SYSTEM_SERVICE_READ_FROM_FILE, move |ctx, req, resp| {
            instance.read_from_file(ctx, req, resp)
        });
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_FILE_SYSTEM_SERVICE_WRITE_TO_FILE, move |ctx, req, resp| {
        instance.write_to_file(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder
        .add_client_streaming_handler(&METHOD_FILE_SYSTEM_SERVICE_APPEND_TO_FILE, move |ctx, req, resp| {
            instance.append_to_file(ctx, req, resp)
        });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_FILE_SYSTEM_SERVICE_LIST, move |ctx, req, resp| {
        instance.list(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SYSTEM_SERVICE_GET_ATTRIBUTES, move |ctx, req, resp| {
        instance.get_attributes(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_FILE_SYSTEM_SERVICE_GET_WORKING_DIRECTORY,
        move |ctx, req, resp| instance.get_working_directory(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_FILE_SYSTEM_SERVICE_SET_WORKING_DIRECTORY,
        move |ctx, req, resp| instance.set_working_directory(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_FILE_SYSTEM_SERVICE_SET_POSIX_FILE_PERMISSIONS,
        move |ctx, req, resp| instance.set_posix_file_permissions(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SYSTEM_SERVICE_READ_SYMBOLIC_LINK, move |ctx, req, resp| {
        instance.read_symbolic_link(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SYSTEM_SERVICE_GET_PATH_SEPARATOR, move |ctx, req, resp| {
        instance.get_path_separator(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SYSTEM_SERVICE_IS_OPEN, move |ctx, req, resp| {
        instance.is_open(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SYSTEM_SERVICE_CLOSE, move |ctx, req, resp| {
        instance.close(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SYSTEM_SERVICE_WAIT_UNTIL_DONE, move |ctx, req, resp| {
        instance.wait_until_done(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SYSTEM_SERVICE_LOCAL_FILE_SYSTEMS, move |ctx, req, resp| {
        instance.local_file_systems(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_FILE_SYSTEM_SERVICE_LIST_FILE_SYSTEMS, move |ctx, req, resp| {
        instance.list_file_systems(ctx, req, resp)
    });
    builder.build()
}

const METHOD_SCHEDULER_SERVICE_GET_ADAPTOR_DESCRIPTIONS: ::grpcio::Method<
    super::xenon::Empty,
    super::xenon::SchedulerAdaptorDescriptions,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/xenon.SchedulerService/getAdaptorDescriptions",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SCHEDULER_SERVICE_GET_ADAPTOR_NAMES: ::grpcio::Method<super::xenon::Empty, super::xenon::AdaptorNames> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.SchedulerService/getAdaptorNames",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_SCHEDULER_SERVICE_GET_ADAPTOR_DESCRIPTION: ::grpcio::Method<
    super::xenon::AdaptorName,
    super::xenon::SchedulerAdaptorDescription,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/xenon.SchedulerService/getAdaptorDescription",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SCHEDULER_SERVICE_CREATE: ::grpcio::Method<super::xenon::CreateSchedulerRequest, super::xenon::Scheduler> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.SchedulerService/create",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_SCHEDULER_SERVICE_GET_ADAPTOR_NAME: ::grpcio::Method<super::xenon::Scheduler, super::xenon::AdaptorName> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.SchedulerService/getAdaptorName",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_SCHEDULER_SERVICE_GET_LOCATION: ::grpcio::Method<super::xenon::Scheduler, super::xenon::Location> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.SchedulerService/getLocation",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_SCHEDULER_SERVICE_GET_CREDENTIAL: ::grpcio::Method<
    super::xenon::Scheduler,
    super::xenon::GetCredentialResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/xenon.SchedulerService/getCredential",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SCHEDULER_SERVICE_GET_PROPERTIES: ::grpcio::Method<super::xenon::Scheduler, super::xenon::Properties> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.SchedulerService/getProperties",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_SCHEDULER_SERVICE_SUBMIT_BATCH_JOB: ::grpcio::Method<
    super::xenon::SubmitBatchJobRequest,
    super::xenon::Job,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/xenon.SchedulerService/submitBatchJob",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SCHEDULER_SERVICE_SUBMIT_INTERACTIVE_JOB: ::grpcio::Method<
    super::xenon::SubmitInteractiveJobRequest,
    super::xenon::SubmitInteractiveJobResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/xenon.SchedulerService/submitInteractiveJob",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SCHEDULER_SERVICE_GET_QUEUE_NAMES: ::grpcio::Method<super::xenon::Scheduler, super::xenon::Queues> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.SchedulerService/getQueueNames",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_SCHEDULER_SERVICE_GET_DEFAULT_QUEUE_NAME: ::grpcio::Method<super::xenon::Scheduler, super::xenon::Queue> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.SchedulerService/getDefaultQueueName",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_SCHEDULER_SERVICE_GET_JOBS: ::grpcio::Method<super::xenon::SchedulerAndQueues, super::xenon::Jobs> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.SchedulerService/getJobs",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_SCHEDULER_SERVICE_GET_JOB_STATUS: ::grpcio::Method<super::xenon::JobRequest, super::xenon::JobStatus> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.SchedulerService/getJobStatus",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_SCHEDULER_SERVICE_GET_JOB_STATUSES: ::grpcio::Method<
    super::xenon::GetJobStatusesRequest,
    super::xenon::GetJobStatusesResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/xenon.SchedulerService/getJobStatuses",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SCHEDULER_SERVICE_GET_QUEUE_STATUS: ::grpcio::Method<
    super::xenon::GetQueueStatusRequest,
    super::xenon::QueueStatus,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/xenon.SchedulerService/getQueueStatus",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SCHEDULER_SERVICE_GET_QUEUE_STATUSES: ::grpcio::Method<
    super::xenon::SchedulerAndQueues,
    super::xenon::QueueStatuses,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/xenon.SchedulerService/getQueueStatuses",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SCHEDULER_SERVICE_WAIT_UNTIL_DONE: ::grpcio::Method<super::xenon::WaitRequest, super::xenon::JobStatus> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.SchedulerService/waitUntilDone",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_SCHEDULER_SERVICE_WAIT_UNTIL_RUNNING: ::grpcio::Method<
    super::xenon::WaitRequest,
    super::xenon::JobStatus,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/xenon.SchedulerService/waitUntilRunning",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SCHEDULER_SERVICE_IS_OPEN: ::grpcio::Method<super::xenon::Scheduler, super::xenon::Is> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.SchedulerService/isOpen",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_SCHEDULER_SERVICE_CANCEL_JOB: ::grpcio::Method<super::xenon::JobRequest, super::xenon::JobStatus> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.SchedulerService/cancelJob",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_SCHEDULER_SERVICE_GET_DEFAULT_RUNTIME: ::grpcio::Method<
    super::xenon::Scheduler,
    super::xenon::GetDefaultRuntimeResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/xenon.SchedulerService/getDefaultRuntime",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_SCHEDULER_SERVICE_GET_FILE_SYSTEM: ::grpcio::Method<super::xenon::Scheduler, super::xenon::FileSystem> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.SchedulerService/getFileSystem",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_SCHEDULER_SERVICE_CLOSE: ::grpcio::Method<super::xenon::Scheduler, super::xenon::Empty> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.SchedulerService/close",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_SCHEDULER_SERVICE_LOCAL_SCHEDULER: ::grpcio::Method<super::xenon::Empty, super::xenon::Scheduler> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.SchedulerService/localScheduler",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_SCHEDULER_SERVICE_LIST_SCHEDULERS: ::grpcio::Method<super::xenon::Empty, super::xenon::Schedulers> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/xenon.SchedulerService/listSchedulers",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

#[derive(Clone)]
pub struct SchedulerServiceClient {
    client: ::grpcio::Client,
}

impl SchedulerServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        SchedulerServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_adaptor_descriptions_opt(
        &self,
        req: &super::xenon::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::SchedulerAdaptorDescriptions> {
        self.client
            .unary_call(&METHOD_SCHEDULER_SERVICE_GET_ADAPTOR_DESCRIPTIONS, req, opt)
    }

    pub fn get_adaptor_descriptions(
        &self,
        req: &super::xenon::Empty,
    ) -> ::grpcio::Result<super::xenon::SchedulerAdaptorDescriptions> {
        self.get_adaptor_descriptions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_adaptor_descriptions_async_opt(
        &self,
        req: &super::xenon::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::SchedulerAdaptorDescriptions>> {
        self.client
            .unary_call_async(&METHOD_SCHEDULER_SERVICE_GET_ADAPTOR_DESCRIPTIONS, req, opt)
    }

    pub fn get_adaptor_descriptions_async(
        &self,
        req: &super::xenon::Empty,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::SchedulerAdaptorDescriptions>> {
        self.get_adaptor_descriptions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_adaptor_names_opt(
        &self,
        req: &super::xenon::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::AdaptorNames> {
        self.client
            .unary_call(&METHOD_SCHEDULER_SERVICE_GET_ADAPTOR_NAMES, req, opt)
    }

    pub fn get_adaptor_names(
        &self,
        req: &super::xenon::Empty,
    ) -> ::grpcio::Result<super::xenon::AdaptorNames> {
        self.get_adaptor_names_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_adaptor_names_async_opt(
        &self,
        req: &super::xenon::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::AdaptorNames>> {
        self.client
            .unary_call_async(&METHOD_SCHEDULER_SERVICE_GET_ADAPTOR_NAMES, req, opt)
    }

    pub fn get_adaptor_names_async(
        &self,
        req: &super::xenon::Empty,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::AdaptorNames>> {
        self.get_adaptor_names_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_adaptor_description_opt(
        &self,
        req: &super::xenon::AdaptorName,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::SchedulerAdaptorDescription> {
        self.client
            .unary_call(&METHOD_SCHEDULER_SERVICE_GET_ADAPTOR_DESCRIPTION, req, opt)
    }

    pub fn get_adaptor_description(
        &self,
        req: &super::xenon::AdaptorName,
    ) -> ::grpcio::Result<super::xenon::SchedulerAdaptorDescription> {
        self.get_adaptor_description_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_adaptor_description_async_opt(
        &self,
        req: &super::xenon::AdaptorName,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::SchedulerAdaptorDescription>> {
        self.client
            .unary_call_async(&METHOD_SCHEDULER_SERVICE_GET_ADAPTOR_DESCRIPTION, req, opt)
    }

    pub fn get_adaptor_description_async(
        &self,
        req: &super::xenon::AdaptorName,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::SchedulerAdaptorDescription>> {
        self.get_adaptor_description_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_opt(
        &self,
        req: &super::xenon::CreateSchedulerRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Scheduler> {
        self.client.unary_call(&METHOD_SCHEDULER_SERVICE_CREATE, req, opt)
    }

    pub fn create(
        &self,
        req: &super::xenon::CreateSchedulerRequest,
    ) -> ::grpcio::Result<super::xenon::Scheduler> {
        self.create_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_async_opt(
        &self,
        req: &super::xenon::CreateSchedulerRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Scheduler>> {
        self.client.unary_call_async(&METHOD_SCHEDULER_SERVICE_CREATE, req, opt)
    }

    pub fn create_async(
        &self,
        req: &super::xenon::CreateSchedulerRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Scheduler>> {
        self.create_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_adaptor_name_opt(
        &self,
        req: &super::xenon::Scheduler,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::AdaptorName> {
        self.client
            .unary_call(&METHOD_SCHEDULER_SERVICE_GET_ADAPTOR_NAME, req, opt)
    }

    pub fn get_adaptor_name(
        &self,
        req: &super::xenon::Scheduler,
    ) -> ::grpcio::Result<super::xenon::AdaptorName> {
        self.get_adaptor_name_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_adaptor_name_async_opt(
        &self,
        req: &super::xenon::Scheduler,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::AdaptorName>> {
        self.client
            .unary_call_async(&METHOD_SCHEDULER_SERVICE_GET_ADAPTOR_NAME, req, opt)
    }

    pub fn get_adaptor_name_async(
        &self,
        req: &super::xenon::Scheduler,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::AdaptorName>> {
        self.get_adaptor_name_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_location_opt(
        &self,
        req: &super::xenon::Scheduler,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Location> {
        self.client.unary_call(&METHOD_SCHEDULER_SERVICE_GET_LOCATION, req, opt)
    }

    pub fn get_location(
        &self,
        req: &super::xenon::Scheduler,
    ) -> ::grpcio::Result<super::xenon::Location> {
        self.get_location_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_location_async_opt(
        &self,
        req: &super::xenon::Scheduler,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Location>> {
        self.client
            .unary_call_async(&METHOD_SCHEDULER_SERVICE_GET_LOCATION, req, opt)
    }

    pub fn get_location_async(
        &self,
        req: &super::xenon::Scheduler,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Location>> {
        self.get_location_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_credential_opt(
        &self,
        req: &super::xenon::Scheduler,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::GetCredentialResponse> {
        self.client
            .unary_call(&METHOD_SCHEDULER_SERVICE_GET_CREDENTIAL, req, opt)
    }

    pub fn get_credential(
        &self,
        req: &super::xenon::Scheduler,
    ) -> ::grpcio::Result<super::xenon::GetCredentialResponse> {
        self.get_credential_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_credential_async_opt(
        &self,
        req: &super::xenon::Scheduler,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::GetCredentialResponse>> {
        self.client
            .unary_call_async(&METHOD_SCHEDULER_SERVICE_GET_CREDENTIAL, req, opt)
    }

    pub fn get_credential_async(
        &self,
        req: &super::xenon::Scheduler,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::GetCredentialResponse>> {
        self.get_credential_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_properties_opt(
        &self,
        req: &super::xenon::Scheduler,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Properties> {
        self.client
            .unary_call(&METHOD_SCHEDULER_SERVICE_GET_PROPERTIES, req, opt)
    }

    pub fn get_properties(
        &self,
        req: &super::xenon::Scheduler,
    ) -> ::grpcio::Result<super::xenon::Properties> {
        self.get_properties_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_properties_async_opt(
        &self,
        req: &super::xenon::Scheduler,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Properties>> {
        self.client
            .unary_call_async(&METHOD_SCHEDULER_SERVICE_GET_PROPERTIES, req, opt)
    }

    pub fn get_properties_async(
        &self,
        req: &super::xenon::Scheduler,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Properties>> {
        self.get_properties_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn submit_batch_job_opt(
        &self,
        req: &super::xenon::SubmitBatchJobRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Job> {
        self.client
            .unary_call(&METHOD_SCHEDULER_SERVICE_SUBMIT_BATCH_JOB, req, opt)
    }

    pub fn submit_batch_job(
        &self,
        req: &super::xenon::SubmitBatchJobRequest,
    ) -> ::grpcio::Result<super::xenon::Job> {
        self.submit_batch_job_opt(req, ::grpcio::CallOption::default())
    }

    pub fn submit_batch_job_async_opt(
        &self,
        req: &super::xenon::SubmitBatchJobRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Job>> {
        self.client
            .unary_call_async(&METHOD_SCHEDULER_SERVICE_SUBMIT_BATCH_JOB, req, opt)
    }

    pub fn submit_batch_job_async(
        &self,
        req: &super::xenon::SubmitBatchJobRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Job>> {
        self.submit_batch_job_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn submit_interactive_job_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<super::xenon::SubmitInteractiveJobRequest>,
        ::grpcio::ClientDuplexReceiver<super::xenon::SubmitInteractiveJobResponse>,
    )> {
        self.client
            .duplex_streaming(&METHOD_SCHEDULER_SERVICE_SUBMIT_INTERACTIVE_JOB, opt)
    }

    pub fn submit_interactive_job(
        &self
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<super::xenon::SubmitInteractiveJobRequest>,
        ::grpcio::ClientDuplexReceiver<super::xenon::SubmitInteractiveJobResponse>,
    )> {
        self.submit_interactive_job_opt(::grpcio::CallOption::default())
    }

    pub fn get_queue_names_opt(
        &self,
        req: &super::xenon::Scheduler,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Queues> {
        self.client
            .unary_call(&METHOD_SCHEDULER_SERVICE_GET_QUEUE_NAMES, req, opt)
    }

    pub fn get_queue_names(
        &self,
        req: &super::xenon::Scheduler,
    ) -> ::grpcio::Result<super::xenon::Queues> {
        self.get_queue_names_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_queue_names_async_opt(
        &self,
        req: &super::xenon::Scheduler,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Queues>> {
        self.client
            .unary_call_async(&METHOD_SCHEDULER_SERVICE_GET_QUEUE_NAMES, req, opt)
    }

    pub fn get_queue_names_async(
        &self,
        req: &super::xenon::Scheduler,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Queues>> {
        self.get_queue_names_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_default_queue_name_opt(
        &self,
        req: &super::xenon::Scheduler,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Queue> {
        self.client
            .unary_call(&METHOD_SCHEDULER_SERVICE_GET_DEFAULT_QUEUE_NAME, req, opt)
    }

    pub fn get_default_queue_name(
        &self,
        req: &super::xenon::Scheduler,
    ) -> ::grpcio::Result<super::xenon::Queue> {
        self.get_default_queue_name_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_default_queue_name_async_opt(
        &self,
        req: &super::xenon::Scheduler,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Queue>> {
        self.client
            .unary_call_async(&METHOD_SCHEDULER_SERVICE_GET_DEFAULT_QUEUE_NAME, req, opt)
    }

    pub fn get_default_queue_name_async(
        &self,
        req: &super::xenon::Scheduler,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Queue>> {
        self.get_default_queue_name_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_jobs_opt(
        &self,
        req: &super::xenon::SchedulerAndQueues,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Jobs> {
        self.client.unary_call(&METHOD_SCHEDULER_SERVICE_GET_JOBS, req, opt)
    }

    pub fn get_jobs(
        &self,
        req: &super::xenon::SchedulerAndQueues,
    ) -> ::grpcio::Result<super::xenon::Jobs> {
        self.get_jobs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_jobs_async_opt(
        &self,
        req: &super::xenon::SchedulerAndQueues,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Jobs>> {
        self.client
            .unary_call_async(&METHOD_SCHEDULER_SERVICE_GET_JOBS, req, opt)
    }

    pub fn get_jobs_async(
        &self,
        req: &super::xenon::SchedulerAndQueues,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Jobs>> {
        self.get_jobs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_job_status_opt(
        &self,
        req: &super::xenon::JobRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::JobStatus> {
        self.client
            .unary_call(&METHOD_SCHEDULER_SERVICE_GET_JOB_STATUS, req, opt)
    }

    pub fn get_job_status(
        &self,
        req: &super::xenon::JobRequest,
    ) -> ::grpcio::Result<super::xenon::JobStatus> {
        self.get_job_status_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_job_status_async_opt(
        &self,
        req: &super::xenon::JobRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::JobStatus>> {
        self.client
            .unary_call_async(&METHOD_SCHEDULER_SERVICE_GET_JOB_STATUS, req, opt)
    }

    pub fn get_job_status_async(
        &self,
        req: &super::xenon::JobRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::JobStatus>> {
        self.get_job_status_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_job_statuses_opt(
        &self,
        req: &super::xenon::GetJobStatusesRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::GetJobStatusesResponse> {
        self.client
            .unary_call(&METHOD_SCHEDULER_SERVICE_GET_JOB_STATUSES, req, opt)
    }

    pub fn get_job_statuses(
        &self,
        req: &super::xenon::GetJobStatusesRequest,
    ) -> ::grpcio::Result<super::xenon::GetJobStatusesResponse> {
        self.get_job_statuses_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_job_statuses_async_opt(
        &self,
        req: &super::xenon::GetJobStatusesRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::GetJobStatusesResponse>> {
        self.client
            .unary_call_async(&METHOD_SCHEDULER_SERVICE_GET_JOB_STATUSES, req, opt)
    }

    pub fn get_job_statuses_async(
        &self,
        req: &super::xenon::GetJobStatusesRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::GetJobStatusesResponse>> {
        self.get_job_statuses_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_queue_status_opt(
        &self,
        req: &super::xenon::GetQueueStatusRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::QueueStatus> {
        self.client
            .unary_call(&METHOD_SCHEDULER_SERVICE_GET_QUEUE_STATUS, req, opt)
    }

    pub fn get_queue_status(
        &self,
        req: &super::xenon::GetQueueStatusRequest,
    ) -> ::grpcio::Result<super::xenon::QueueStatus> {
        self.get_queue_status_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_queue_status_async_opt(
        &self,
        req: &super::xenon::GetQueueStatusRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::QueueStatus>> {
        self.client
            .unary_call_async(&METHOD_SCHEDULER_SERVICE_GET_QUEUE_STATUS, req, opt)
    }

    pub fn get_queue_status_async(
        &self,
        req: &super::xenon::GetQueueStatusRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::QueueStatus>> {
        self.get_queue_status_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_queue_statuses_opt(
        &self,
        req: &super::xenon::SchedulerAndQueues,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::QueueStatuses> {
        self.client
            .unary_call(&METHOD_SCHEDULER_SERVICE_GET_QUEUE_STATUSES, req, opt)
    }

    pub fn get_queue_statuses(
        &self,
        req: &super::xenon::SchedulerAndQueues,
    ) -> ::grpcio::Result<super::xenon::QueueStatuses> {
        self.get_queue_statuses_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_queue_statuses_async_opt(
        &self,
        req: &super::xenon::SchedulerAndQueues,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::QueueStatuses>> {
        self.client
            .unary_call_async(&METHOD_SCHEDULER_SERVICE_GET_QUEUE_STATUSES, req, opt)
    }

    pub fn get_queue_statuses_async(
        &self,
        req: &super::xenon::SchedulerAndQueues,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::QueueStatuses>> {
        self.get_queue_statuses_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn wait_until_done_opt(
        &self,
        req: &super::xenon::WaitRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::JobStatus> {
        self.client
            .unary_call(&METHOD_SCHEDULER_SERVICE_WAIT_UNTIL_DONE, req, opt)
    }

    pub fn wait_until_done(
        &self,
        req: &super::xenon::WaitRequest,
    ) -> ::grpcio::Result<super::xenon::JobStatus> {
        self.wait_until_done_opt(req, ::grpcio::CallOption::default())
    }

    pub fn wait_until_done_async_opt(
        &self,
        req: &super::xenon::WaitRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::JobStatus>> {
        self.client
            .unary_call_async(&METHOD_SCHEDULER_SERVICE_WAIT_UNTIL_DONE, req, opt)
    }

    pub fn wait_until_done_async(
        &self,
        req: &super::xenon::WaitRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::JobStatus>> {
        self.wait_until_done_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn wait_until_running_opt(
        &self,
        req: &super::xenon::WaitRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::JobStatus> {
        self.client
            .unary_call(&METHOD_SCHEDULER_SERVICE_WAIT_UNTIL_RUNNING, req, opt)
    }

    pub fn wait_until_running(
        &self,
        req: &super::xenon::WaitRequest,
    ) -> ::grpcio::Result<super::xenon::JobStatus> {
        self.wait_until_running_opt(req, ::grpcio::CallOption::default())
    }

    pub fn wait_until_running_async_opt(
        &self,
        req: &super::xenon::WaitRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::JobStatus>> {
        self.client
            .unary_call_async(&METHOD_SCHEDULER_SERVICE_WAIT_UNTIL_RUNNING, req, opt)
    }

    pub fn wait_until_running_async(
        &self,
        req: &super::xenon::WaitRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::JobStatus>> {
        self.wait_until_running_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn is_open_opt(
        &self,
        req: &super::xenon::Scheduler,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Is> {
        self.client.unary_call(&METHOD_SCHEDULER_SERVICE_IS_OPEN, req, opt)
    }

    pub fn is_open(
        &self,
        req: &super::xenon::Scheduler,
    ) -> ::grpcio::Result<super::xenon::Is> {
        self.is_open_opt(req, ::grpcio::CallOption::default())
    }

    pub fn is_open_async_opt(
        &self,
        req: &super::xenon::Scheduler,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Is>> {
        self.client
            .unary_call_async(&METHOD_SCHEDULER_SERVICE_IS_OPEN, req, opt)
    }

    pub fn is_open_async(
        &self,
        req: &super::xenon::Scheduler,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Is>> {
        self.is_open_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_job_opt(
        &self,
        req: &super::xenon::JobRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::JobStatus> {
        self.client.unary_call(&METHOD_SCHEDULER_SERVICE_CANCEL_JOB, req, opt)
    }

    pub fn cancel_job(
        &self,
        req: &super::xenon::JobRequest,
    ) -> ::grpcio::Result<super::xenon::JobStatus> {
        self.cancel_job_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_job_async_opt(
        &self,
        req: &super::xenon::JobRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::JobStatus>> {
        self.client
            .unary_call_async(&METHOD_SCHEDULER_SERVICE_CANCEL_JOB, req, opt)
    }

    pub fn cancel_job_async(
        &self,
        req: &super::xenon::JobRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::JobStatus>> {
        self.cancel_job_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_default_runtime_opt(
        &self,
        req: &super::xenon::Scheduler,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::GetDefaultRuntimeResponse> {
        self.client
            .unary_call(&METHOD_SCHEDULER_SERVICE_GET_DEFAULT_RUNTIME, req, opt)
    }

    pub fn get_default_runtime(
        &self,
        req: &super::xenon::Scheduler,
    ) -> ::grpcio::Result<super::xenon::GetDefaultRuntimeResponse> {
        self.get_default_runtime_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_default_runtime_async_opt(
        &self,
        req: &super::xenon::Scheduler,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::GetDefaultRuntimeResponse>> {
        self.client
            .unary_call_async(&METHOD_SCHEDULER_SERVICE_GET_DEFAULT_RUNTIME, req, opt)
    }

    pub fn get_default_runtime_async(
        &self,
        req: &super::xenon::Scheduler,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::GetDefaultRuntimeResponse>> {
        self.get_default_runtime_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_file_system_opt(
        &self,
        req: &super::xenon::Scheduler,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::FileSystem> {
        self.client
            .unary_call(&METHOD_SCHEDULER_SERVICE_GET_FILE_SYSTEM, req, opt)
    }

    pub fn get_file_system(
        &self,
        req: &super::xenon::Scheduler,
    ) -> ::grpcio::Result<super::xenon::FileSystem> {
        self.get_file_system_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_file_system_async_opt(
        &self,
        req: &super::xenon::Scheduler,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::FileSystem>> {
        self.client
            .unary_call_async(&METHOD_SCHEDULER_SERVICE_GET_FILE_SYSTEM, req, opt)
    }

    pub fn get_file_system_async(
        &self,
        req: &super::xenon::Scheduler,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::FileSystem>> {
        self.get_file_system_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn close_opt(
        &self,
        req: &super::xenon::Scheduler,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Empty> {
        self.client.unary_call(&METHOD_SCHEDULER_SERVICE_CLOSE, req, opt)
    }

    pub fn close(
        &self,
        req: &super::xenon::Scheduler,
    ) -> ::grpcio::Result<super::xenon::Empty> {
        self.close_opt(req, ::grpcio::CallOption::default())
    }

    pub fn close_async_opt(
        &self,
        req: &super::xenon::Scheduler,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Empty>> {
        self.client.unary_call_async(&METHOD_SCHEDULER_SERVICE_CLOSE, req, opt)
    }

    pub fn close_async(
        &self,
        req: &super::xenon::Scheduler,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Empty>> {
        self.close_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn local_scheduler_opt(
        &self,
        req: &super::xenon::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Scheduler> {
        self.client
            .unary_call(&METHOD_SCHEDULER_SERVICE_LOCAL_SCHEDULER, req, opt)
    }

    pub fn local_scheduler(
        &self,
        req: &super::xenon::Empty,
    ) -> ::grpcio::Result<super::xenon::Scheduler> {
        self.local_scheduler_opt(req, ::grpcio::CallOption::default())
    }

    pub fn local_scheduler_async_opt(
        &self,
        req: &super::xenon::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Scheduler>> {
        self.client
            .unary_call_async(&METHOD_SCHEDULER_SERVICE_LOCAL_SCHEDULER, req, opt)
    }

    pub fn local_scheduler_async(
        &self,
        req: &super::xenon::Empty,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Scheduler>> {
        self.local_scheduler_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_schedulers_opt(
        &self,
        req: &super::xenon::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::xenon::Schedulers> {
        self.client
            .unary_call(&METHOD_SCHEDULER_SERVICE_LIST_SCHEDULERS, req, opt)
    }

    pub fn list_schedulers(
        &self,
        req: &super::xenon::Empty,
    ) -> ::grpcio::Result<super::xenon::Schedulers> {
        self.list_schedulers_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_schedulers_async_opt(
        &self,
        req: &super::xenon::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Schedulers>> {
        self.client
            .unary_call_async(&METHOD_SCHEDULER_SERVICE_LIST_SCHEDULERS, req, opt)
    }

    pub fn list_schedulers_async(
        &self,
        req: &super::xenon::Empty,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::xenon::Schedulers>> {
        self.list_schedulers_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(
        &self,
        f: F,
    ) where
        F: ::futures::Future<Output = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}

pub trait SchedulerService {
    fn get_adaptor_descriptions(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::Empty,
        sink: ::grpcio::UnarySink<super::xenon::SchedulerAdaptorDescriptions>,
    );
    fn get_adaptor_names(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::Empty,
        sink: ::grpcio::UnarySink<super::xenon::AdaptorNames>,
    );
    fn get_adaptor_description(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::AdaptorName,
        sink: ::grpcio::UnarySink<super::xenon::SchedulerAdaptorDescription>,
    );
    fn create(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::CreateSchedulerRequest,
        sink: ::grpcio::UnarySink<super::xenon::Scheduler>,
    );
    fn get_adaptor_name(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::Scheduler,
        sink: ::grpcio::UnarySink<super::xenon::AdaptorName>,
    );
    fn get_location(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::Scheduler,
        sink: ::grpcio::UnarySink<super::xenon::Location>,
    );
    fn get_credential(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::Scheduler,
        sink: ::grpcio::UnarySink<super::xenon::GetCredentialResponse>,
    );
    fn get_properties(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::Scheduler,
        sink: ::grpcio::UnarySink<super::xenon::Properties>,
    );
    fn submit_batch_job(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::SubmitBatchJobRequest,
        sink: ::grpcio::UnarySink<super::xenon::Job>,
    );
    fn submit_interactive_job(
        &mut self,
        ctx: ::grpcio::RpcContext,
        stream: ::grpcio::RequestStream<super::xenon::SubmitInteractiveJobRequest>,
        sink: ::grpcio::DuplexSink<super::xenon::SubmitInteractiveJobResponse>,
    );
    fn get_queue_names(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::Scheduler,
        sink: ::grpcio::UnarySink<super::xenon::Queues>,
    );
    fn get_default_queue_name(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::Scheduler,
        sink: ::grpcio::UnarySink<super::xenon::Queue>,
    );
    fn get_jobs(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::SchedulerAndQueues,
        sink: ::grpcio::UnarySink<super::xenon::Jobs>,
    );
    fn get_job_status(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::JobRequest,
        sink: ::grpcio::UnarySink<super::xenon::JobStatus>,
    );
    fn get_job_statuses(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::GetJobStatusesRequest,
        sink: ::grpcio::UnarySink<super::xenon::GetJobStatusesResponse>,
    );
    fn get_queue_status(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::GetQueueStatusRequest,
        sink: ::grpcio::UnarySink<super::xenon::QueueStatus>,
    );
    fn get_queue_statuses(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::SchedulerAndQueues,
        sink: ::grpcio::UnarySink<super::xenon::QueueStatuses>,
    );
    fn wait_until_done(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::WaitRequest,
        sink: ::grpcio::UnarySink<super::xenon::JobStatus>,
    );
    fn wait_until_running(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::WaitRequest,
        sink: ::grpcio::UnarySink<super::xenon::JobStatus>,
    );
    fn is_open(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::Scheduler,
        sink: ::grpcio::UnarySink<super::xenon::Is>,
    );
    fn cancel_job(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::JobRequest,
        sink: ::grpcio::UnarySink<super::xenon::JobStatus>,
    );
    fn get_default_runtime(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::Scheduler,
        sink: ::grpcio::UnarySink<super::xenon::GetDefaultRuntimeResponse>,
    );
    fn get_file_system(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::Scheduler,
        sink: ::grpcio::UnarySink<super::xenon::FileSystem>,
    );
    fn close(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::Scheduler,
        sink: ::grpcio::UnarySink<super::xenon::Empty>,
    );
    fn local_scheduler(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::Empty,
        sink: ::grpcio::UnarySink<super::xenon::Scheduler>,
    );
    fn list_schedulers(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::xenon::Empty,
        sink: ::grpcio::UnarySink<super::xenon::Schedulers>,
    );
}

pub fn create_scheduler_service<S: SchedulerService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SCHEDULER_SERVICE_GET_ADAPTOR_DESCRIPTIONS,
        move |ctx, req, resp| instance.get_adaptor_descriptions(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_SERVICE_GET_ADAPTOR_NAMES, move |ctx, req, resp| {
        instance.get_adaptor_names(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SCHEDULER_SERVICE_GET_ADAPTOR_DESCRIPTION,
        move |ctx, req, resp| instance.get_adaptor_description(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_SERVICE_CREATE, move |ctx, req, resp| {
        instance.create(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_SERVICE_GET_ADAPTOR_NAME, move |ctx, req, resp| {
        instance.get_adaptor_name(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_SERVICE_GET_LOCATION, move |ctx, req, resp| {
        instance.get_location(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_SERVICE_GET_CREDENTIAL, move |ctx, req, resp| {
        instance.get_credential(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_SERVICE_GET_PROPERTIES, move |ctx, req, resp| {
        instance.get_properties(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_SERVICE_SUBMIT_BATCH_JOB, move |ctx, req, resp| {
        instance.submit_batch_job(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(
        &METHOD_SCHEDULER_SERVICE_SUBMIT_INTERACTIVE_JOB,
        move |ctx, req, resp| instance.submit_interactive_job(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_SERVICE_GET_QUEUE_NAMES, move |ctx, req, resp| {
        instance.get_queue_names(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_SCHEDULER_SERVICE_GET_DEFAULT_QUEUE_NAME,
        move |ctx, req, resp| instance.get_default_queue_name(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_SERVICE_GET_JOBS, move |ctx, req, resp| {
        instance.get_jobs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_SERVICE_GET_JOB_STATUS, move |ctx, req, resp| {
        instance.get_job_status(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_SERVICE_GET_JOB_STATUSES, move |ctx, req, resp| {
        instance.get_job_statuses(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_SERVICE_GET_QUEUE_STATUS, move |ctx, req, resp| {
        instance.get_queue_status(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_SERVICE_GET_QUEUE_STATUSES, move |ctx, req, resp| {
        instance.get_queue_statuses(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_SERVICE_WAIT_UNTIL_DONE, move |ctx, req, resp| {
        instance.wait_until_done(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_SERVICE_WAIT_UNTIL_RUNNING, move |ctx, req, resp| {
        instance.wait_until_running(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_SERVICE_IS_OPEN, move |ctx, req, resp| {
        instance.is_open(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_SERVICE_CANCEL_JOB, move |ctx, req, resp| {
        instance.cancel_job(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_SERVICE_GET_DEFAULT_RUNTIME, move |ctx, req, resp| {
        instance.get_default_runtime(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_SERVICE_GET_FILE_SYSTEM, move |ctx, req, resp| {
        instance.get_file_system(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_SERVICE_CLOSE, move |ctx, req, resp| {
        instance.close(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_SERVICE_LOCAL_SCHEDULER, move |ctx, req, resp| {
        instance.local_scheduler(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_SERVICE_LIST_SCHEDULERS, move |ctx, req, resp| {
        instance.list_schedulers(ctx, req, resp)
    });
    builder.build()
}
