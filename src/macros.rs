
/// A macro wrapping the LightCli parse function. 
#[macro_export]
macro_rules! lightcli {
    ($cli:expr, $cmd:ident, $key:ident, $val:ident, [ 
        $(
            $cmdv:pat => [
                $( $keyv:pat => $action:expr ),*
            ] => $done:expr
        );*
    ], $nomatch1:expr, $nomatch2:expr, $nomatch3:expr) => {
        let _ = $cli.parse_data(|cbcmd| {
            match cbcmd {
                $crate::CallbackCommand::Attribute($cmd, $key, $val) => {
                    match $cmd {
                        $(
                        $cmdv => {
                            match $key {
                                $(
                                    $keyv => { $action },
                                )*
                                _ => $nomatch2,
                            }
                        }
                        )*
                        _ => $nomatch3,
                    }
                },
                $crate::CallbackCommand::Command($cmd) => {
                    match $cmd {
                        $(
                            $cmdv => $done,
                        )*
                        _ => $nomatch3,
                    }
                }
            }
        });
    };
}
