#[derive(Properties, PartialEq)]
pub struct AccountCtxProps {
    pub id: String,
    pub route: Route,
}

#[function_component(AccountCtx)]
pub fn account_ctx<T>(props: &AccountCtxProps) -> HtmlResult {
    let account_url = format!("/api/account/{}", props.id);
    let account: UseFutureHandle<Result<rmmt::Account, _>> =
        use_future(|| async move { utils::get(&account_url).await })?;
    let account: &rmmt::Account = match *account {
        Ok(ref res) => res,
        Err(ref error) => return Ok(html! { <FetchError error={ format!("{:?}", error) } /> }),
    };

    let users_url = format!("/api/account/{}/users", props.id);
    let users: UseFutureHandle<Result<Vec<rmmt::User>, _>> =
        use_future(|| async move { utils::get(&users_url).await })?;
    let users: HashMap<Uuid, rmmt::User> = match *users {
        Ok(ref res) => res.iter().cloned().map(|u| (u.id.clone(), u)).collect(),
        Err(ref error) => return Ok(html! { <FetchError error={ format!("{:?}", error) } /> }),
    };

    let balance_url = format!("/api/account/{}/balance", props.id);
    let balance: UseFutureHandle<Result<rmmt::Balance, _>> =
        use_future(|| async move { utils::get(&balance_url).await })?;
    let balance: &rmmt::Balance = match *balance {
        Ok(ref res) => res,
        Err(ref error) => return Ok(html! { <FetchError error={ format!("{:?}", error) } /> }),
    };

    let update_users = callback!(async move || {
        let users_url = format!("/api/account/{}/users", props.id);
        let users: Result<Vec<rmmt::User>, _> =
            use_future(|| async move { utils::get(&users_url).await })?;
        todo!("update user");
    });
    let update_balance = callback!();

    Ok(html! {
        <T {account} {users} {balance} {update_users} {update_balance} />
    })
}
