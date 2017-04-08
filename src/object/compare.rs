use object::method;


pub trait RichComparison
    : method::Equal + method::NotEqual + method::LessThan + method::LessOrEqual + method::GreaterThan + method::GreaterOrEqual
    {
}
