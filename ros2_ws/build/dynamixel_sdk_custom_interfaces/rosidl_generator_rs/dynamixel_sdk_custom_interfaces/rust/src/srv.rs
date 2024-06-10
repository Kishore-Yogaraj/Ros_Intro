

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPosition_Request {
    pub id: u8,
}



impl Default for GetPosition_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::GetPosition_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetPosition_Request {
  type RmwMsg = crate::srv::rmw::GetPosition_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id: msg.id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id,
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPosition_Response {
    pub position: i32,
}



impl Default for GetPosition_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::GetPosition_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetPosition_Response {
  type RmwMsg = crate::srv::rmw::GetPosition_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        position: msg.position,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      position: msg.position,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      position: msg.position,
    }
  }
}






#[link(name = "dynamixel_sdk_custom_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dynamixel_sdk_custom_interfaces__srv__GetPosition() -> *const std::os::raw::c_void;
}

// Corresponds to dynamixel_sdk_custom_interfaces__srv__GetPosition
pub struct GetPosition;

impl rosidl_runtime_rs::Service for GetPosition {
  type Request = crate::srv::GetPosition_Request;
  type Response = crate::srv::GetPosition_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__dynamixel_sdk_custom_interfaces__srv__GetPosition() }
  }
}



pub mod rmw {
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[link(name = "dynamixel_sdk_custom_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dynamixel_sdk_custom_interfaces__srv__GetPosition_Request() -> *const std::os::raw::c_void;
}

#[link(name = "dynamixel_sdk_custom_interfaces__rosidl_generator_c")]
extern "C" {
    fn dynamixel_sdk_custom_interfaces__srv__GetPosition_Request__init(msg: *mut GetPosition_Request) -> bool;
    fn dynamixel_sdk_custom_interfaces__srv__GetPosition_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPosition_Request>, size: usize) -> bool;
    fn dynamixel_sdk_custom_interfaces__srv__GetPosition_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPosition_Request>);
    fn dynamixel_sdk_custom_interfaces__srv__GetPosition_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPosition_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPosition_Request>) -> bool;
}

// Corresponds to dynamixel_sdk_custom_interfaces__srv__GetPosition_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPosition_Request {
    pub id: u8,
}



impl Default for GetPosition_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dynamixel_sdk_custom_interfaces__srv__GetPosition_Request__init(&mut msg as *mut _) {
        panic!("Call to dynamixel_sdk_custom_interfaces__srv__GetPosition_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPosition_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dynamixel_sdk_custom_interfaces__srv__GetPosition_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dynamixel_sdk_custom_interfaces__srv__GetPosition_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dynamixel_sdk_custom_interfaces__srv__GetPosition_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPosition_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPosition_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dynamixel_sdk_custom_interfaces/srv/GetPosition_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dynamixel_sdk_custom_interfaces__srv__GetPosition_Request() }
  }
}


#[link(name = "dynamixel_sdk_custom_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dynamixel_sdk_custom_interfaces__srv__GetPosition_Response() -> *const std::os::raw::c_void;
}

#[link(name = "dynamixel_sdk_custom_interfaces__rosidl_generator_c")]
extern "C" {
    fn dynamixel_sdk_custom_interfaces__srv__GetPosition_Response__init(msg: *mut GetPosition_Response) -> bool;
    fn dynamixel_sdk_custom_interfaces__srv__GetPosition_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPosition_Response>, size: usize) -> bool;
    fn dynamixel_sdk_custom_interfaces__srv__GetPosition_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPosition_Response>);
    fn dynamixel_sdk_custom_interfaces__srv__GetPosition_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPosition_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPosition_Response>) -> bool;
}

// Corresponds to dynamixel_sdk_custom_interfaces__srv__GetPosition_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPosition_Response {
    pub position: i32,
}



impl Default for GetPosition_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dynamixel_sdk_custom_interfaces__srv__GetPosition_Response__init(&mut msg as *mut _) {
        panic!("Call to dynamixel_sdk_custom_interfaces__srv__GetPosition_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPosition_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dynamixel_sdk_custom_interfaces__srv__GetPosition_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dynamixel_sdk_custom_interfaces__srv__GetPosition_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dynamixel_sdk_custom_interfaces__srv__GetPosition_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPosition_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPosition_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dynamixel_sdk_custom_interfaces/srv/GetPosition_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dynamixel_sdk_custom_interfaces__srv__GetPosition_Response() }
  }
}






  #[link(name = "dynamixel_sdk_custom_interfaces__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__dynamixel_sdk_custom_interfaces__srv__GetPosition() -> *const std::os::raw::c_void;
  }

  // Corresponds to dynamixel_sdk_custom_interfaces__srv__GetPosition
  pub struct GetPosition;

  impl rosidl_runtime_rs::Service for GetPosition {
    type Request = crate::srv::rmw::GetPosition_Request;
    type Response = crate::srv::rmw::GetPosition_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__dynamixel_sdk_custom_interfaces__srv__GetPosition() }
    }
  }



}  // mod rmw
