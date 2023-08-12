use crate::{
    ipc::{client_id::BYTE_LENGTH, AlignmentContext, RpcKind},
    security::UserGroupInformation,
};
use hadoop_proto::hadoop::common::{
    rpc_request_header_proto::OperationProto, IpcConnectionContextProto, RpcKindProto,
    RpcRequestHeaderProto, UserInformationProto,
};
use std::rc::Rc;

pub struct ProtoUtil;

impl ProtoUtil {
    pub fn make_ipc_connection_context(
        protocol: Option<&str>,
        ugi: Option<&UserGroupInformation>,
        auth_method: &str,
    ) -> IpcConnectionContextProto {
        let mut ugi_proto = UserInformationProto {
            effective_user: None,
            real_user: None,
        };
        if let Some(ugi) = ugi {
            if auth_method == "kerberos" {
                // Real user was established as part of the connection.
                // Send effective user only.
                ugi_proto.effective_user = Some(ugi.get_user_name());
            } else if auth_method == "token" {
                // With token, the connection itself establishes
                // both real and effective user. Hence send none in header.
            } else {
                // Simple authentication
                // No user info is established as part of the connection.
                // Send both effective user and real user
                ugi_proto.effective_user = Some(ugi.get_user_name());
            }
        }
        IpcConnectionContextProto {
            user_info: Some(ugi_proto),
            protocol: protocol.map(|p| p.to_string()),
        }
    }

    pub fn convert<S, T: From<S>>(value: S) -> T {
        T::from(value)
    }

    pub fn make_rpc_request_header(
        rpc_kind: &RpcKind,
        operation: OperationProto,
        call_id: i32,
        retry_count: i32,
        uuid: &[u8; BYTE_LENGTH],
        alignment_context: Option<Rc<dyn AlignmentContext>>,
    ) -> RpcRequestHeaderProto {
        let result = RpcRequestHeaderProto {
            rpc_kind: Some(RpcKindProto::from(rpc_kind).into()),
            rpc_op: Some(operation.into()),
            call_id,
            client_id: uuid.to_vec(),
            retry_count: Some(retry_count),
            trace_info: None,
            caller_context: None,
            state_id: None,
            router_federated_state: None,
        };

        // Add alignment context if it is not null
        if let Some(_alignment_context) = alignment_context {
            // TODO
        }

        result
    }
}

impl From<&RpcKind> for RpcKindProto {
    fn from(kind: &RpcKind) -> Self {
        match kind {
            RpcKind::RpcBuiltin => RpcKindProto::RpcBuiltin,
            RpcKind::RpcWritable => RpcKindProto::RpcWritable,
            RpcKind::RpcProtocolBuffer => RpcKindProto::RpcProtocolBuffer,
        }
    }
}
