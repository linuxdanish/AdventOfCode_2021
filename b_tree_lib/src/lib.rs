#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

}

pub struct Node<T> {
    pub value: Box<T>,
    pub count: i32,
    pub l: Option<Box<Node<T>>>,
    pub r: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Self {
            value: Box::new(val),
            count: 1,
            l: None,
            r: None
        }
    }
}
