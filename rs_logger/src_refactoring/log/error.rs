use crate::log_fn::log_error;

pub fn log_error_source(code: u32) -> String {
  let code = code / 100;
}

// === LEGEND === //
// Code[log_type:0][crate_id:00][module_id:000][error_id: 00]
// 
// 10 > rs_logger
//  |- 001 > ::init::
// a > rs_logger
// |- 001 > init
// 0000000
// 000000
// 1502599
// f02b4a

pub enum LogErrorSource {
    Code01001
}
impl LogErrorSource {
    pub fn source(self) -> String {
        let source = match self {
          Code01001
        };

        source.to_owned()
    }
}

pub enum LogError {
    //
}
impl LogError {
    pub fn log(self) {
      let rep = "2b3";
      let as_hex = u32::from_str_radix(rep, 16).unwrap();


        match self {
          // 
      }
    }
}
