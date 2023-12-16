mod tree;


fn main() {
   let mut hardtry : tree::Tree = tree::Tree::new();
   hardtry.insert(5);
   hardtry.insert(2);
   hardtry.insert(4);
   hardtry.insert(5);
   hardtry.insert(7);
   hardtry.insert(9);
   hardtry.insert(10);
   hardtry.insert(1);

   println!("{}", hardtry);

   let ima_stupid : tree::Tree = tree::Tree::new();
   println!("{}", ima_stupid);
}
