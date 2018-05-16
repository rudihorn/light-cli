
/// This macro allows for an easy way to define key value commands while
/// still allowing to define custom error handlers.
/// 
/// # Arguments 
/// * `$cli`: The [`LightCliInput`] instance to parse data from.
/// * `$cmd`: The identifier to use to access the current command.
/// * `$key`: The identifier to use to access the curernt key.
/// * `$val`: The identifier to use to access the curernt value.
/// * `$cmdv`: The name of the command.
/// * `$keyv`: The key for command `$cmdv`.
/// * `$action`: What to do with the value `$val` for the given command and key.
/// * `$done`: What to do when the command is complete.
/// * `$nomatch1`: What to do when the command value is not found 
///             while trying to find a key action.
/// * `$nomatch2`: What to do when the key value is not found
///             while trying to find a key action.
/// * `$nomatch3`: What to do when the command value is not found
///             while trying to execute a command.
/// 
/// [`LightCliInput`]: struct.LightCliInput.html
/// 
/// # Remarks
/// For a simpler way to write a command see the macro [`lightcli!`].
/// This macro makes use of the underlying function [`parse_data`].
/// 
/// [`lightcli_adv!`]: macro.lightcli_adv.html
/// [`parse_data`]: struct.LightCliInput.html#method.parse_data
#[macro_export]
macro_rules! lightcli_adv {
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
                        _ => $nomatch1,
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


/// This macro allows for an easy way to define key value commands.
/// 
/// # Arguments 
/// * `$cli`: The [`LightCliInput`] instance to parse data from.
/// * `$cl_out`: The [`LightCliOutput`] instance to write errors to.
/// * `$cmd`: The identifier to use to access the current command.
/// * `$key`: The identifier to use to access the curernt key.
/// * `$val`: The identifier to use to access the curernt value.
/// * `$cmdv`: The name of the command.
/// * `$keyv`: The key for command `$cmdv`.
/// * `$action`: What to do with the value `$val` for the given command and key.
/// * `$done`: What to do when the command is complete.
/// 
/// [`LightCliInput`]: struct.LightCliInput.html
/// [`LightCliOutput`]: struct.LightCliOutput.html
/// 
/// # Remarks
/// For a command that doesn't use the output and allows for custom 
/// error handling see the macro [`lightcli_adv!`]. This macro makes use
/// of the underlying function [`parse_data`].
/// 
/// [`lightcli_adv!`]: macro.lightcli_adv.html
/// [`parse_data`]: struct.LightCliInput.html#method.parse_data
#[macro_export]
macro_rules! lightcli {
    ($cli:expr, $cl_out:expr, $cmd:ident, $key:ident, $val:ident, [ 
        $(
            $cmdv:pat => [
                $( $keyv:pat => $action:expr ),*
            ] => $done:expr
        );*
    ]) => {
        lightcli_adv!($cli, $cmd, $key, $val, [
                $(
                    $cmdv => [
                        $( $keyv => $action ),*
                    ] => $done
                );*
            ], 
            {}, 
            {writeln!($cl_out, "Unknown key for command {}: {}", $cmd, $key).unwrap()}, 
            {writeln!($cl_out, "Unknown command: {}", $cmd).unwrap()}
        );
    };
}