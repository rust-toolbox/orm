
pub trait Identifiable
{
    type ID;

    fn id(&self) -> Self::ID;
    fn set_id(&mut self, id: Self::ID) -> &mut Self;
}

pub trait Connection
{

}

pub trait Query
{
    type Entity: Identifiable;

    fn from<C: Connection>(db: C) -> Self;
    fn one(&mut self) -> Self::Entity;
    fn all(&self) -> Vec<Self::Entity>;
    fn by(&mut self) -> &Self;
    fn limit(&self, limit: u32) -> Self;
    fn offset(&self, offset: u32) -> Self;
    fn count(&self) -> u32;
    fn exists(&self) -> bool;
}

pub trait DataMapper<'a, ID, Q: Query>
{
    type Entity: Identifiable<ID = ID>;

    fn find() -> Q;
    fn at(entity: &'a mut Self::Entity) -> Self;
    fn create(&mut self) -> bool;
    //fn create(entity: &mut Self::Entity) -> bool;
    fn update(&self) -> u32;
    fn save(&self) -> bool;
    fn delete(&self) -> u32;
}