#[macro_export]
macro_rules! address_of {
    ($account:ident) => {
        ink_e2e::account_id(ink_e2e::AccountKeyring::$account)
    };
}

#[macro_export]
macro_rules! balance_of {
    ($client:ident, $call:ident, $account:ident) => {{
        let _msg = $call.balance_of(address_of!($account));
        $client
            .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
            .await
            .return_value()
    }};
}

#[macro_export]
macro_rules! owner_of {
    ($client:ident, $call:ident, $id:expr) => {{
        let _msg = $call.contract.owner_of($id);
        $client
            .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
            .await
            .return_value()
    }};
}

#[macro_export]
macro_rules! balance_of_37 {
    ($client:ident, $call:ident, $account:ident, $token:expr) => {{
        let _msg = $call.balance_of(address_of!($account), $token);
        $client
            .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
            .await
            .return_value()
    }};
}

#[macro_export]
macro_rules! has_role {
    ($client:ident, $call:ident, $role:expr, $account:ident) => {{
        let _msg = $call.has_role($role, Some(address_of!($account)));
        $client
            .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
            .await
            .return_value()
    }};
}

#[macro_export]
macro_rules! grant_role {
    ($client:ident, $call:ident, $role:expr, $account:ident) => {{
        let _msg = $call.grant_role($role, Some(address_of!($account)));
        $client
            .call(&ink_e2e::alice(), &_msg, 0, None)
            .await.expect("grant_role failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! revoke_role {
    ($client:ident, $call:ident, $role:expr, $account:ident) => {{
        let _msg = $call.revoke_role($role, Some(address_of!($account)));
        $client
            .call(&ink_e2e::alice(), &_msg, 0, None)
            .await
            .expect("revoke_role failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! mint_dry_run {
    ($client:ident, $call:ident, $account:ident, $id:expr) => {{
        let _msg = $call.mint(address_of!($account), $id);
        $client
            .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
            .await
            .return_value()
    }};
    ($client:ident, $call:ident, $signer:ident, $account:ident, $id:expr) => {{
        let _msg = $call.mint(address_of!($account), $id);
        $client
            .call_dry_run(&ink_e2e::$signer(), &_msg, 0, None)
            .await
            .return_value()
    }};
}

#[macro_export]
macro_rules! mint {
    ($client:ident, $call:ident, $account:ident, $id:expr) => {{
        let _msg = $call.mint(address_of!($account), $id);
        $client
            .call(&ink_e2e::alice(), &_msg, 0, None)
            .await
            .expect("mint failed")
            .return_value()
    }};
    ($client:ident, $call:ident, $signer:ident, $account:ident, $id:expr) => {{
        let _msg = $call.mint(address_of!($account), $id);
        $client
            .call(&ink_e2e::$signer(), &_msg, 0, None)
            .await
            .expect("mint failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! get_role_member_count {
    ($client:ident, $call:ident, $role:expr) => {{
        let _msg = $call.get_role_member_count($role);
        $client
            .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
            .await
            .return_value()
    }};
}

#[macro_export]
macro_rules! get_role_member {
    ($client:ident, $call:ident, $role:expr, $index:expr) => {{
        let _msg = $call.get_role_member($role, $index);
        $client
            .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
            .await
            .return_value()
    }};
}

#[macro_export]
macro_rules! get_shares {
    ($client:ident, $call:ident, $user:ident) => {{
        let _msg = $call.shares(address_of!($user));
        $client
            .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
            .await
            .return_value()
    }};
}

#[macro_export]
macro_rules! method_call {
    ($client:ident, $call:ident, $method:ident) => {{
        let _msg = $call.$method();
        $client
            .call(&ink_e2e::alice(), _msg, 0, None)
            .await
            .expect("call failed")
            .return_value()
    }};
    ($client:ident, $call:ident, $signer:ident, $method:ident) => {{
        let _msg = $call.$method();
        $client
            .call(&ink_e2e::$signer(), _msg, 0, None)
            .await
            .expect("call failed")
            .return_value()
    }};
    ($client:ident, $call:ident, $method:ident($($args:expr),*)) => {{
        let _msg = $call.$method($($args),*);
        $client
            .call(&ink_e2e::alice(), &_msg, 0, None)
            .await
            .expect("call failed")
            .return_value()
    }};
    ($client:ident, $call:ident, $signer:ident, $method:ident($($args:expr),*)) => {{
        let _msg = $call.$method($($args),*);
        $client
            .call(&ink_e2e::$signer(), &_msg, 0, None)
            .await
            .expect("call failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! method_call_dry_run {
    ($client:ident, $call:ident, $method:ident) => {{
        let msg = $call.$method;
        $client
            .call_dry_run(&ink_e2e::alice(), &msg, 0, None)
            .await
            .return_value()
    }};
    ($client:ident, $call:ident, $method:ident($($args:expr),*)) => {{
        let _msg = $call.$method($($args),*);
        $client
            .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
            .await
            .return_value()
    }};
    ($client:ident, $call:ident, $signer:ident, $method:ident) => {{
        let _msg = $call.$method();
        $client
            .call_dry_run(&ink_e2e::$signer(), &_msg, 0, None)
            .await
            .return_value()
    }};
    ($client:ident, $call:ident, $signer:ident, $method:ident($($args:expr),*)) => {{
        let _msg = $call.$method($($args),*);
        $client
            .call_dry_run(&ink_e2e::$signer(), &_msg, 0, None)
            .await
            .return_value()
    }};
}
