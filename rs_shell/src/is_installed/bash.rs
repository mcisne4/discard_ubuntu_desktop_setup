use super::_utils;

pub fn bash() -> Result<bool, String> {
    match _utils::command_v_cmd("bash") {
        Ok((code, _stdout, _stderr)) => {
            if code == 0 {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(e) => return Err(e),
    }
}
