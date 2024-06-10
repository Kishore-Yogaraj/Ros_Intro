pub mod rmw {
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[link(name = "dynamixel_sdk_custom_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dynamixel_sdk_custom_interfaces__msg__SetPosition() -> *const std::os::raw::c_void;
}

#[link(name = "dynamixel_sdk_custom_interfaces__rosidl_generator_c")]
extern "C" {
    fn dynamixel_sdk_custom_interfaces__msg__SetPosition__init(msg: *mut SetPosition) -> bool;
    fn dynamixel_sdk_custom_interfaces__msg__SetPosition__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetPosition>, size: usize) -> bool;
    fn dynamixel_sdk_custom_interfaces__msg__SetPosition__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetPosition>);
    fn dynamixel_sdk_custom_interfaces__msg__SetPosition__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetPosition>, out_seq: *mut rosidl_runtime_rs::Sequence<SetPosition>) -> bool;
}

// Corresponds to dynamixel_sdk_custom_interfaces__msg__SetPosition
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPosition {
    pub id: u8,
    pub position: i32,
}



impl Default for SetPosition {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dynamixel_sdk_custom_interfaces__msg__SetPosition__init(&mut msg as *mut _) {
        panic!("Call to dynamixel_sdk_custom_interfaces__msg__SetPosition__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetPosition {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dynamixel_sdk_custom_interfaces__msg__SetPosition__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dynamixel_sdk_custom_interfaces__msg__SetPosition__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dynamixel_sdk_custom_interfaces__msg__SetPosition__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetPosition {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetPosition where Self: Sized {
  const TYPE_NAME: &'static str = "dynamixel_sdk_custom_interfaces/msg/SetPosition";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dynamixel_sdk_custom_interfaces__msg__SetPosition() }
  }
}


}  // mod rmw


#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPosition {
    pub id: u8,
    pub position: i32,
}



impl Default for SetPosition {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::SetPosition::default())
  }
}

impl rosidl_runtime_rs::Message for SetPosition {
  type RmwMsg = crate::msg::rmw::SetPosition;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id,
        position: msg.position,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id: msg.id,
      position: msg.position,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id,
      position: msg.position,
    }
  }
}


