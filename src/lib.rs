#[allow(dead_code)]
struct RangeTree {
    value: i32,
    left: Option<Box<RangeTree>>,
    right: Option<Box<RangeTree>>,
    leftsum: i32,
    rightsum: i32,
}

#[allow(dead_code)]
impl RangeTree {

    fn new(value: i32) -> RangeTree {
        RangeTree {
            value: value,
            left: None,
            right: None,
            leftsum: 0,
            rightsum: 0,
        }
    }

    fn insert(&mut self, value: i32) {
        if value < self.value {
            
            if let Some(ref mut child) = self.left {
                child.insert(value);
            } else {
                self.left = Some(Box::new(RangeTree::new(value))); 
            }
        } else {
            self.rightsum += 1;
            
            if let Some(ref mut child) = self.right {
                child.insert(value);
            } else {
                self.right = Some(Box::new(RangeTree::new(value))); 
            }
        }
    }

    fn get_range(self, pred: i32, succ: i32) -> Vec<i32> {
        let mut range = Vec::new();

        if pred <= self.value && self.value < succ {
            range.insert(value);
        };

        // Sum left children
        if pred <= self.value {
            if let Some(left) = self.left {
                summation += self.rightsum + left.sum(pred, succ);
            };
        } else if succ > self.value {
            if let Some(right) = self.right {
                summation += self.leftsum + right.sum(pred, succ);
            };
        };
        
        summation
    }
}

#[cfg(test)]
mod tests {
    use RangeTree;

    #[test]
    fn it_works() {
        let mut tree = RangeTree::new(3);
        tree.insert(1);
        tree.insert(2);
        tree.insert(4);
        tree.insert(5);
        println!("Sum: {}", tree.sum(-9999, 9999));
    }
}
