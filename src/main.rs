pub trait HKT<A, B> {
    type URI;
    type Target;
}

pub trait Functor<A, B>: HKT<A, B> {
    fn fmap<F>(self, f: F) -> <Self as HKT<A, B>>::Target
        where F: FnOnce(A) -> B;
}

pub trait Chain<A, B>: HKT<A, B> {
    fn chain<F>(self, f: F) -> <Self as HKT<A, B>>::Target
        where F: FnOnce(A) -> <Self as HKT<A, B>>::Target;
}

pub trait HKT3<A, B, C> {
    type Target2;
}

trait Apply<A, F, B> : Functor<A, B> + HKT3<A, F, B>
    where F: FnOnce(A) -> B,
{
    fn ap(self, f: <Self as HKT3<A, F, B>>::Target2) -> <Self as HKT<A, B>>::Target;
}

trait Pure<A>: HKT<A, A> {
    fn of(self) -> <Self as HKT<A, A>>::Target;
}




pub enum Either<L, R> {
    Left(L), 
    Right(R)
}

impl<L, R> Either<L, R> {

    fn unwrap(self) -> R {
        match self {
            Either::Left(l) => panic!("left value"),
            Either::Right(r) => r
        }
    }
}

impl <C, A, B> HKT<A, B> for Either<C, A> {
    type URI = Self;
    type Target = Either<C, B>;
}

impl <C, A, B> Functor<A, B> for Either<C, A> {
    fn fmap<F>(self, f: F) -> <Self as HKT<A, B>>::Target
        where F: FnOnce(A) -> B {
            match self {
                Either::Left(x) => Either::Left(x), 
                Either::Right(x) => Either::Right(f(x))
            }
        }
}

impl<C, A, B> Chain<A, B> for Either<C, A> {
    fn chain<F>(self, f: F) -> Self::Target
        where F: FnOnce(A) -> <Self as HKT<A, B>>::Target {
            match self {
                Either::Left(x) => Either::Left(x), 
                Either::Right(x) => f(x)
            }
    }
}

impl <D, A, B, C> HKT3<A, B, C> for Either<D, A> {
    type Target2 = Either<D, B>;
}

impl<C, A, F, B> Apply<A, F, B> for Either<C, A>
    where F: FnOnce(A) -> B,
{
    fn ap(self, f: Self::Target2) -> Self::Target {
        
        match self {
            Either::Left(l) => Either::Left(l), 
            Either::Right(r) => f.fmap(|z| z(r))
        }
    }
}







fn main() {
    let x: Either<String, i32> = Either::Right(10);
    let y = x.chain(|x| {
        Either::Right(x+10)
    }).unwrap();
    println!("{0}", y);
}
