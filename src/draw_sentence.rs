use std::f64::consts::TAU;
use crate::draw_word::draw_plain_word;
use crate::shape::{BShape, Cart, Circle, Polar, ShapeSet, Shapes,Thickness::*};
use crate::tree::{Number, Sentence, Word,WordTypes,WordTypes::*};


fn get_word_rad(sen_rad:f64,num_words:u32) -> (f64, f64,f64){
    let diff = TAU/num_words as f64;
    let max_word_radius = match num_words {
        0 => 0.0,
        1 => sen_rad,
        _ => sen_rad/(1f64+(2f64/(1f64-diff.cos())).sqrt())
    };

    let word_radius = max_word_radius/1.6;
    return (word_radius, max_word_radius,diff);
}
fn draw_word(word: &WordTypes, this:&Polar,last:&Polar,diff:f64,word_radius:f64) -> Result<(BShape,Polar),String> {
    match word{
        PlainWord(word) => {
            let word = Word::try_from(word.clone())?;
            let cart_pos:Cart = Cart::from(this.clone());
            let mut these_shapes = draw_plain_word(&word, this,word_radius);
            these_shapes.shove(cart_pos);
            Ok((these_shapes,this.rotate(diff)))
        },
        crate::tree::WordTypes::Number(word) => {
            let num = &Number::try_from(word.clone())?;
            let cart_pos:Cart = Cart::from(this.clone());
            let mut these_shapes:BShape = (num,word_radius).into();
            these_shapes.shove(cart_pos);
            Ok((these_shapes,this.rotate(diff)))
        },
        Punctuation(word) => {
            todo!()
        }
    }
}

impl TryFrom<(&Sentence,f64)> for BShape {//also return length
    type Error = String;
    fn try_from((sentence,sen_rad):(&Sentence,f64)) -> Result<Self,Self::Error> {
        let num_words = sentence.get_num_words();

        let (word_radius,max_word_radius,diff) = get_word_rad(sen_rad,num_words);
        let inner_radius = sen_rad-max_word_radius;

        let mut pos = Polar::new(inner_radius, -TAU/4.0);
        let mut last = pos.clone();

        let mut shapes:Shapes = Vec::new();

        shapes.push(Box::new(Circle::new(Cart::origin(), sen_rad+2.0*Thick.val(sen_rad),Some(Thick.val(sen_rad)))));
        shapes.push(Box::new(Circle::new(Cart::origin(), sen_rad+(Thick.val(sen_rad)/2.0),Some(Thin.val(sen_rad)))));

        for word in &sentence.words {
            let (new_shapes,new_next) = draw_word(word, &pos, &last, diff,word_radius)?;
            shapes.push(new_shapes);
            last = pos;
            pos = new_next;
        }
        let shape = Box::new(ShapeSet::new(shapes, "sentence"));
        return Ok(shape);
    }
}