use ast::*;
use std::marker::PhantomData;
use swc_common::{Fold, FoldWith};

pub fn noop() -> impl Pass + Clone + Copy {
    #[derive(Clone, Copy)]
    struct Noop;
    impl<T> Fold<T> for Noop
    where
        T: FoldWith<Self>,
    {
        fn fold(&mut self, n: T) -> T {
            n
        }
    }
    Noop
}

macro_rules! mk_impl {
    ($T:ty) => {
        // impl<A: Pass, B: Pass> Fold<$T> for JoinedPass<A, B, $T> {
        //     default fn fold(&mut self, node: $T) -> $T {
        //         self.second.fold(self.first.fold(node))
        //     }
        // }
    };
}

macro_rules! mk_trait {
    ($($T:ty,)*) => {
        /// Crazy trait to make traversal fast again.
        ///
        /// Note that pass.clone() should create a fresh pass.
        pub trait Pass: objekt::Clone + $( ::swc_common::Fold<$T> + )* {}
        impl<P> Pass for P
            where P: ?Sized + objekt::Clone + $( ::swc_common::Fold<$T> +)*{

        }

        $(
            mk_impl!($T);
        )*
    };
}
objekt::clone_trait_object!(Pass);
objekt::clone_trait_object!(Pass + Send + Sync);

mk_trait!(
    // ArrayLit,
    // ArrayPat,
    // ArrowExpr,
    // AssignExpr,
    // AssignPat,
    // AssignPatProp,
    // AssignProp,
    // AwaitExpr,
    // BinExpr,
    // BlockStmt,
    // Bool,
    // BreakStmt,
    // CallExpr,
    // CatchClause,
    // Class,
    // ClassDecl,
    // ClassExpr,
    // Method,
    // CondExpr,
    // ContinueStmt,
    // DebuggerStmt,
    // DoWhileStmt,
    // EmptyStmt,
    // ExportAll,
    // ExportSpecifier,
    // ExprOrSpread,
    // FnDecl,
    // FnExpr,
    // ForInStmt,
    // ForOfStmt,
    // ForStmt,
    // Function,
    // GetterProp,
    // Ident,
    // IfStmt,
    // ImportDecl,
    // ImportDefault,
    // ImportSpecific,
    // ImportStarAs,
    // KeyValuePatProp,
    // KeyValueProp,
    // LabeledStmt,
    // MemberExpr,
    // MetaPropExpr,
    // MethodProp,
    Module,
    /* NamedExport,
     * NewExpr,
     * Null,
     * Number,
     * ObjectLit,
     * ObjectPat,
     * ParenExpr,
     * Regex,
     * RestPat,
     * ReturnStmt,
     * SeqExpr,
     * SetterProp,
     * SpreadElement,
     * Str,
     * SwitchCase,
     * SwitchStmt,
     * ThisExpr,
     * ThrowStmt,
     * TplElement,
     * Tpl,
     * TaggedTpl,
     * TryStmt,
     * UnaryExpr,
     * UpdateExpr,
     * VarDecl,
     * VarDeclarator,
     * WhileStmt,
     * WithStmt,
     * YieldExpr,
     * // enums
     * AssignOp,
     * BinaryOp,
     * BlockStmtOrExpr,
     * MethodKind,
     * Decl,
     * ExportDefaultDecl,
     * Expr,
     * ExprOrSuper,
     * ImportSpecifier,
     * Lit,
     * ModuleDecl,
     * ModuleItem,
     * ObjectPatProp,
     * Pat,
     * PatOrExpr,
     * Prop,
     * PropName,
     * PropOrSpread,
     * Stmt,
     * UnaryOp,
     * UpdateOp,
     * VarDeclKind,
     * VarDeclOrExpr,
     * VarDeclOrPat, */
);

#[derive(Debug, Copy)]
pub struct JoinedPass<A, B, N> {
    pub first: A,
    pub second: B,
    pub ty: PhantomData<N>,
}

impl<A: Pass, B: Pass, N> Clone for JoinedPass<A, B, N> {
    fn clone(&self) -> Self {
        JoinedPass {
            first: objekt::clone(&self.first),
            second: objekt::clone(&self.second),
            ty: self.ty,
        }
    }
}

// fn type_name<T>() -> String {
//     format!("{}", unsafe { std::intrinsics::type_name::<T>() })
// }

impl<A, B, T> Fold<T> for JoinedPass<A, B, T>
where
    T: FoldWith<Self>,
    A: Fold<T>,
    B: Fold<T>,
{
    #[inline(always)]
    fn fold(&mut self, node: T) -> T {
        // println!(
        //     "Folding<{}><{}>({})",
        //     type_name::<A>(),
        //     type_name::<B>(),
        //     type_name::<T>()
        // );
        self.second.fold(self.first.fold(node))
    }
}

impl<A, B, T, N> Fold<T> for JoinedPass<A, B, N>
where
    T: FoldWith<Self>,
{
    #[inline(always)]
    default fn fold(&mut self, node: T) -> T {
        node.fold_children(self)
    }
}
