pub trait Functor {
    type Inner;
    type This<B>: Functor; // この型の中にBを入れた型

    fn fmap<F, B>(self, f: F) -> Self::This<B>
    where
        F: FnOnce(Self::Inner) -> B;
}

pub trait Applicative: Functor {
    fn pure(a: Self::Inner) -> Self::This<Self::Inner>;
    // HaskellのApplicativeの(<*>)と同じ
    fn apply<F, B>(self, f: Self::This<F>) -> Self::This<B>
    where
        F: FnOnce(Self::Inner) -> B;
}

pub trait Monad: Applicative {
    fn bind<F, B>(self, f: F) -> Self::This<B>
    where
        F: FnOnce(Self::Inner) -> Self::This<B>;
    fn return_m(a: Self::Inner) -> Self::This<Self::Inner> {
        Self::pure(a)
    }
}

impl<T> Functor for Option<T> {
    type Inner = T;
    type This<B> = Result<B, ()>;

    fn fmap<F, B>(self, f: F) -> Self::This<B>
    where
        F: FnOnce(Self::Inner) -> B,
    {
        self.map(f).ok_or(())
    }
}

struct OptionF<T>(Option<T>);

impl<T> OptionF<T> {
    fn fmap<F, B>(self, f: F) -> impl Functor<Inner = B, This<B> = Result<B, ()>>
    where
        F: FnOnce(T) -> B,
    {
        self.0.map(f).ok_or(())
    }
}

impl<T> Functor for Result<T, ()> {
    type Inner = T;
    type This<B> = Result<B, ()>;

    fn fmap<F, B>(self, f: F) -> Self::This<B>
    where
        F: FnOnce(Self::Inner) -> B,
    {
        self.map(f)
    }
}
