use std::fmt::Display;

pub struct UserParams {
    pub port: u32,
    pub buffer_size: u32,
}

impl Display for UserParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(port={}, buffer_size={})", self.port, self.buffer_size)
    }
}

impl UserParams {
    pub fn get_user_params(args: Vec<String>) -> UserParams {
        let mut port: u32 = crate::PORT;
        let mut buffer_size: u32 = crate::BUFFER_SIZE;

        for (idx, arg) in args.iter().enumerate() {
            if args.len() == idx + 1 {
                break;
            }
            match arg.as_str() {
                "--port" => {
                    port = match args[idx + 1].parse::<u32>() {
                        Ok(val) => val,
                        Err(_) => port,
                    }
                }
                "--buffer_size" => {
                    buffer_size = match args[idx + 1].parse::<u32>() {
                        Ok(val) => val,
                        Err(_) => buffer_size,
                    }
                }
                _ => (),
            }
        }

        UserParams {
            port: port,
            buffer_size: buffer_size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UserParams;
    #[test]
    fn get_user_params_returns_defaults_scenario_1() {
        let result: UserParams = UserParams::get_user_params(vec![String::from("--port")]);
        assert_eq!(result.port, crate::PORT);
        assert_eq!(result.buffer_size, crate::BUFFER_SIZE);
    }
    #[test]
    fn get_user_params_returns_defaults_scenario_2() {
        let result: UserParams =
            UserParams::get_user_params(vec![String::from("--buffer_size=9090")]);
        assert_eq!(result.port, crate::PORT);
        assert_eq!(result.buffer_size, crate::BUFFER_SIZE);
    }
    #[test]
    fn get_user_params_returns_defaults_scenario_3() {
        let result: UserParams = UserParams::get_user_params(vec![
            String::from("--buffer_size"),
            String::from("9090"),
            String::from("--port"),
        ]);
        assert_eq!(result.port, crate::PORT);
        assert_eq!(result.buffer_size, 9090);
    }
    #[test]
    fn get_user_params_returns_defaults_scenario_4() {
        let result: UserParams = UserParams::get_user_params(vec![
            String::from("--buffer_size"),
            String::from("9090"),
            String::from("--port"),
            String::from("zxc"),
        ]);
        assert_eq!(result.port, crate::PORT);
        assert_eq!(result.buffer_size, 9090);
    }
    #[test]
    fn get_user_params_returns_user_specified_parameters_on_good_input() {
        let result: UserParams = UserParams::get_user_params(vec![
            String::from("--buffer_size"),
            String::from("9090"),
            String::from("--port"),
            String::from("1010"),
        ]);
        assert_eq!(result.buffer_size, 9090);
        assert_eq!(result.port, 1010);
    }
}
