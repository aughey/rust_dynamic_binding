use anyhow::Result;
use std::marker::PhantomData;

pub trait Arguments {
    fn get_index(&self, index: usize) -> Option<&Box<dyn std::any::Any>>;
}

pub trait DynamicCallable {
    fn call(&self, arguments: &dyn Arguments) -> Result<Box<dyn std::any::Any>>;
}

pub trait DynamicIntrospectable {
    fn get_argument_type(&self, index: usize) -> Option<std::any::TypeId>;
    fn get_return_type(&self) -> Option<std::any::TypeId>;
}

struct DynamicFunction1<F, Arg, Out>(F, PhantomData<Arg>, PhantomData<Out>)
where
    F: Fn(&Arg) -> Out + 'static,
    Arg: 'static,
    Out: std::any::Any + 'static;

impl<F, Arg, Out> DynamicCallable for DynamicFunction1<F, Arg, Out>
where
    F: Fn(&Arg) -> Out + 'static,
    Arg: 'static,
    Out: std::any::Any + 'static,
{
    fn call(&self, arguments: &dyn Arguments) -> Result<Box<dyn std::any::Any>> {
        let arg = arguments
            .get_index(0)
            .ok_or_else(|| anyhow::anyhow!("Could not get argument at index 0"))?;

        let typed_arg = arg
            .downcast_ref::<Arg>()
            .ok_or_else(|| anyhow::anyhow!("Could not downcast argument 0 to required type"))?;

        Ok(Box::new(self.0(typed_arg)))
    }
}

impl<F, Arg, Out> DynamicIntrospectable for DynamicFunction1<F, Arg, Out>
where
    F: Fn(&Arg) -> Out + 'static,
    Arg: 'static,
    Out: std::any::Any + 'static,
{
    fn get_argument_type(&self, index: usize) -> Option<std::any::TypeId> {
        if index == 0 {
            Some(std::any::TypeId::of::<Arg>())
        } else {
            None
        }
    }

    fn get_return_type(&self) -> Option<std::any::TypeId> {
        Some(std::any::TypeId::of::<Out>())
    }
}

// Helper function to create a DynamicFunction
pub fn make_dynamic_1<F, Arg, Out>(f: F) -> impl DynamicCallable + DynamicIntrospectable
where
    F: Fn(&Arg) -> Out + 'static,
    Arg: 'static,
    Out: std::any::Any + 'static,
{
    DynamicFunction1(f, PhantomData, PhantomData)
}

impl Arguments for &[Box<dyn std::any::Any>] {
    fn get_index(&self, index: usize) -> Option<&Box<dyn std::any::Any>> {
        self.get(index)
    }
}

// Dynamic function with 0 arguments
struct DynamicFunction0<F, Out>(F, PhantomData<Out>)
where
    F: Fn() -> Out + 'static,
    Out: std::any::Any + 'static;

impl<F, Out> DynamicCallable for DynamicFunction0<F, Out>
where
    F: Fn() -> Out + 'static,
    Out: std::any::Any + 'static,
{
    fn call(&self, _arguments: &dyn Arguments) -> Result<Box<dyn std::any::Any>> {
        Ok(Box::new(self.0()))
    }
}

impl<F, Out> DynamicIntrospectable for DynamicFunction0<F, Out>
where
    F: Fn() -> Out + 'static,
    Out: std::any::Any + 'static,
{
    fn get_argument_type(&self, _index: usize) -> Option<std::any::TypeId> {
        None
    }

    fn get_return_type(&self) -> Option<std::any::TypeId> {
        Some(std::any::TypeId::of::<Out>())
    }
}

pub fn make_dynamic_0<F, Out>(f: F) -> impl DynamicCallable + DynamicIntrospectable
where
    F: Fn() -> Out + 'static,
    Out: std::any::Any + 'static,
{
    DynamicFunction0(f, PhantomData)
}

// Dynamic function with 2 arguments
struct DynamicFunction2<F, Arg1, Arg2, Out>(
    F,
    PhantomData<Arg1>,
    PhantomData<Arg2>,
    PhantomData<Out>,
)
where
    F: Fn(&Arg1, &Arg2) -> Out + 'static,
    Arg1: 'static,
    Arg2: 'static,
    Out: std::any::Any + 'static;

impl<F, Arg1, Arg2, Out> DynamicCallable for DynamicFunction2<F, Arg1, Arg2, Out>
where
    F: Fn(&Arg1, &Arg2) -> Out + 'static,
    Arg1: 'static,
    Arg2: 'static,
    Out: std::any::Any + 'static,
{
    fn call(&self, arguments: &dyn Arguments) -> Result<Box<dyn std::any::Any>> {
        let arg1 = arguments
            .get_index(0)
            .ok_or_else(|| anyhow::anyhow!("Could not get argument at index 0"))?;
        let arg2 = arguments
            .get_index(1)
            .ok_or_else(|| anyhow::anyhow!("Could not get argument at index 1"))?;

        let typed_arg1 = arg1
            .downcast_ref::<Arg1>()
            .ok_or_else(|| anyhow::anyhow!("Could not downcast argument 0 to required type"))?;
        let typed_arg2 = arg2
            .downcast_ref::<Arg2>()
            .ok_or_else(|| anyhow::anyhow!("Could not downcast argument 1 to required type"))?;

        Ok(Box::new(self.0(typed_arg1, typed_arg2)))
    }
}

impl<F, Arg1, Arg2, Out> DynamicIntrospectable for DynamicFunction2<F, Arg1, Arg2, Out>
where
    F: Fn(&Arg1, &Arg2) -> Out + 'static,
    Arg1: 'static,
    Arg2: 'static,
    Out: std::any::Any + 'static,
{
    fn get_argument_type(&self, index: usize) -> Option<std::any::TypeId> {
        match index {
            0 => Some(std::any::TypeId::of::<Arg1>()),
            1 => Some(std::any::TypeId::of::<Arg2>()),
            _ => None,
        }
    }

    fn get_return_type(&self) -> Option<std::any::TypeId> {
        Some(std::any::TypeId::of::<Out>())
    }
}

pub fn make_dynamic_2<F, Arg1, Arg2, Out>(f: F) -> impl DynamicCallable + DynamicIntrospectable
where
    F: Fn(&Arg1, &Arg2) -> Out + 'static,
    Arg1: 'static,
    Arg2: 'static,
    Out: std::any::Any + 'static,
{
    DynamicFunction2(f, PhantomData, PhantomData, PhantomData)
}

// Dynamic function with 3 arguments
struct DynamicFunction3<F, Arg1, Arg2, Arg3, Out>(
    F,
    PhantomData<Arg1>,
    PhantomData<Arg2>,
    PhantomData<Arg3>,
    PhantomData<Out>,
)
where
    F: Fn(&Arg1, &Arg2, &Arg3) -> Out + 'static,
    Arg1: 'static,
    Arg2: 'static,
    Arg3: 'static,
    Out: std::any::Any + 'static;

impl<F, Arg1, Arg2, Arg3, Out> DynamicCallable for DynamicFunction3<F, Arg1, Arg2, Arg3, Out>
where
    F: Fn(&Arg1, &Arg2, &Arg3) -> Out + 'static,
    Arg1: 'static,
    Arg2: 'static,
    Arg3: 'static,
    Out: std::any::Any + 'static,
{
    fn call(&self, arguments: &dyn Arguments) -> Result<Box<dyn std::any::Any>> {
        let arg1 = arguments
            .get_index(0)
            .ok_or_else(|| anyhow::anyhow!("Could not get argument at index 0"))?;
        let arg2 = arguments
            .get_index(1)
            .ok_or_else(|| anyhow::anyhow!("Could not get argument at index 1"))?;
        let arg3 = arguments
            .get_index(2)
            .ok_or_else(|| anyhow::anyhow!("Could not get argument at index 2"))?;

        let typed_arg1 = arg1
            .downcast_ref::<Arg1>()
            .ok_or_else(|| anyhow::anyhow!("Could not downcast argument 0 to required type"))?;
        let typed_arg2 = arg2
            .downcast_ref::<Arg2>()
            .ok_or_else(|| anyhow::anyhow!("Could not downcast argument 1 to required type"))?;
        let typed_arg3 = arg3
            .downcast_ref::<Arg3>()
            .ok_or_else(|| anyhow::anyhow!("Could not downcast argument 2 to required type"))?;

        Ok(Box::new(self.0(typed_arg1, typed_arg2, typed_arg3)))
    }
}

impl<F, Arg1, Arg2, Arg3, Out> DynamicIntrospectable for DynamicFunction3<F, Arg1, Arg2, Arg3, Out>
where
    F: Fn(&Arg1, &Arg2, &Arg3) -> Out + 'static,
    Arg1: 'static,
    Arg2: 'static,
    Arg3: 'static,
    Out: std::any::Any + 'static,
{
    fn get_argument_type(&self, index: usize) -> Option<std::any::TypeId> {
        match index {
            0 => Some(std::any::TypeId::of::<Arg1>()),
            1 => Some(std::any::TypeId::of::<Arg2>()),
            2 => Some(std::any::TypeId::of::<Arg3>()),
            _ => None,
        }
    }

    fn get_return_type(&self) -> Option<std::any::TypeId> {
        Some(std::any::TypeId::of::<Out>())
    }
}

pub fn make_dynamic_3<F, Arg1, Arg2, Arg3, Out>(
    f: F,
) -> impl DynamicCallable + DynamicIntrospectable
where
    F: Fn(&Arg1, &Arg2, &Arg3) -> Out + 'static,
    Arg1: 'static,
    Arg2: 'static,
    Arg3: 'static,
    Out: std::any::Any + 'static,
{
    DynamicFunction3(f, PhantomData, PhantomData, PhantomData, PhantomData)
}

// Dynamic function with 4 arguments
struct DynamicFunction4<F, Arg1, Arg2, Arg3, Arg4, Out>(
    F,
    PhantomData<Arg1>,
    PhantomData<Arg2>,
    PhantomData<Arg3>,
    PhantomData<Arg4>,
    PhantomData<Out>,
)
where
    F: Fn(&Arg1, &Arg2, &Arg3, &Arg4) -> Out + 'static,
    Arg1: 'static,
    Arg2: 'static,
    Arg3: 'static,
    Arg4: 'static,
    Out: std::any::Any + 'static;

impl<F, Arg1, Arg2, Arg3, Arg4, Out> DynamicCallable
    for DynamicFunction4<F, Arg1, Arg2, Arg3, Arg4, Out>
where
    F: Fn(&Arg1, &Arg2, &Arg3, &Arg4) -> Out + 'static,
    Arg1: 'static,
    Arg2: 'static,
    Arg3: 'static,
    Arg4: 'static,
    Out: std::any::Any + 'static,
{
    fn call(&self, arguments: &dyn Arguments) -> Result<Box<dyn std::any::Any>> {
        let arg1 = arguments
            .get_index(0)
            .ok_or_else(|| anyhow::anyhow!("Could not get argument at index 0"))?;
        let arg2 = arguments
            .get_index(1)
            .ok_or_else(|| anyhow::anyhow!("Could not get argument at index 1"))?;
        let arg3 = arguments
            .get_index(2)
            .ok_or_else(|| anyhow::anyhow!("Could not get argument at index 2"))?;
        let arg4 = arguments
            .get_index(3)
            .ok_or_else(|| anyhow::anyhow!("Could not get argument at index 3"))?;

        let typed_arg1 = arg1
            .downcast_ref::<Arg1>()
            .ok_or_else(|| anyhow::anyhow!("Could not downcast argument 0 to required type"))?;
        let typed_arg2 = arg2
            .downcast_ref::<Arg2>()
            .ok_or_else(|| anyhow::anyhow!("Could not downcast argument 1 to required type"))?;
        let typed_arg3 = arg3
            .downcast_ref::<Arg3>()
            .ok_or_else(|| anyhow::anyhow!("Could not downcast argument 2 to required type"))?;
        let typed_arg4 = arg4
            .downcast_ref::<Arg4>()
            .ok_or_else(|| anyhow::anyhow!("Could not downcast argument 3 to required type"))?;

        Ok(Box::new(self.0(
            typed_arg1, typed_arg2, typed_arg3, typed_arg4,
        )))
    }
}

impl<F, Arg1, Arg2, Arg3, Arg4, Out> DynamicIntrospectable
    for DynamicFunction4<F, Arg1, Arg2, Arg3, Arg4, Out>
where
    F: Fn(&Arg1, &Arg2, &Arg3, &Arg4) -> Out + 'static,
    Arg1: 'static,
    Arg2: 'static,
    Arg3: 'static,
    Arg4: 'static,
    Out: std::any::Any + 'static,
{
    fn get_argument_type(&self, index: usize) -> Option<std::any::TypeId> {
        match index {
            0 => Some(std::any::TypeId::of::<Arg1>()),
            1 => Some(std::any::TypeId::of::<Arg2>()),
            2 => Some(std::any::TypeId::of::<Arg3>()),
            3 => Some(std::any::TypeId::of::<Arg4>()),
            _ => None,
        }
    }

    fn get_return_type(&self) -> Option<std::any::TypeId> {
        Some(std::any::TypeId::of::<Out>())
    }
}

pub fn make_dynamic_4<F, Arg1, Arg2, Arg3, Arg4, Out>(
    f: F,
) -> impl DynamicCallable + DynamicIntrospectable
where
    F: Fn(&Arg1, &Arg2, &Arg3, &Arg4) -> Out + 'static,
    Arg1: 'static,
    Arg2: 'static,
    Arg3: 'static,
    Arg4: 'static,
    Out: std::any::Any + 'static,
{
    DynamicFunction4(
        f,
        PhantomData,
        PhantomData,
        PhantomData,
        PhantomData,
        PhantomData,
    )
}
