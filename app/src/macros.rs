/// 资源路由宏：自动注册 RESTful 资源路由
#[macro_export]
macro_rules! resources {
    // ===== 入口 =====
    ($router:expr, $path:expr, $mod:path, [$($action:ident),* $(,)?]) => {{
        use $mod as __res_mod;

        // 分别构建集合路由和单资源路由的 MethodRouter
        let __mr_coll = resources!(@coll __res_mod $($action) *);
        let __mr_single = resources!(@single __res_mod $($action) *);

        let __resource = ::axum::Router::new()
            .route("/", __mr_coll)
            .route("/{id}", __mr_single);

        $router = $router.nest($path, __resource);
        $router
    }};

    // ========================
    // 集合路由
    // ========================
    (@coll $mod:ident) => { ::axum::routing::MethodRouter::new() };
    (@coll $mod:ident index $($rest:tt)*) => {{
        let __mr = resources!(@coll $mod $($rest)*);
        __mr.get($mod::index)
    }};
    (@coll $mod:ident create $($rest:tt)*) => {{
        let __mr = resources!(@coll $mod $($rest)*);
        __mr.post($mod::create)
    }};
    (@coll $mod:ident show $($rest:tt)*) => {{
        resources!(@coll $mod $($rest)*)
    }};
    (@coll $mod:ident update $($rest:tt)*) => {{
        resources!(@coll $mod $($rest)*)
    }};
    (@coll $mod:ident delete $($rest:tt)*) => {{
        resources!(@coll $mod $($rest)*)
    }};

    // ========================
    // 单资源路由
    // ========================
    (@single $mod:ident) => { ::axum::routing::MethodRouter::new() };
    (@single $mod:ident show $($rest:tt)*) => {{
        let __mr = resources!(@single $mod $($rest)*);
        __mr.get($mod::show)
    }};
    (@single $mod:ident update $($rest:tt)*) => {{
        let __mr = resources!(@single $mod $($rest)*);
        __mr.put($mod::update)
    }};
    (@single $mod:ident delete $($rest:tt)*) => {{
        let __mr = resources!(@single $mod $($rest)*);
        __mr.delete($mod::delete)
    }};
    (@single $mod:ident index $($rest:tt)*) => {{
        resources!(@single $mod $($rest)*)
    }};
    (@single $mod:ident create $($rest:tt)*) => {{
        resources!(@single $mod $($rest)*)
    }};
}
