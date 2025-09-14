use std::f64::consts::TAU;
use crate::draw_word::draw_plain_word;
use crate::shape::{BShape, Cart, Circle, Polar, ShapeSet, Shapes, WORD_RADIUS,Thickness::*};
use crate::tree::{Number, Sentence, Word,WordTypes,WordTypes::*};


fn get_inner_outer_diff(num_words:u32) -> (f64,f64,f64) {
    let diff = TAU/num_words as f64;
    let max_word_radius = 1.6*WORD_RADIUS;
    let inner_radius = match num_words {
        0|1 => 0f64,
        2 => max_word_radius,
        n => (2.0*max_word_radius*max_word_radius/(1.0-diff.cos())).sqrt()
    };
    let outer_radius = inner_radius+max_word_radius;
    return (inner_radius, outer_radius, diff);
}
fn draw_word(word: &WordTypes, this:&Polar,last:&Polar,diff:f64) -> Result<(BShape,Polar),String> {
    match word{
        PlainWord(word) => {
            let word = Word::try_from(word.clone())?;
            let cart_pos:Cart = Cart::from(this.clone());
            let mut these_shapes = draw_plain_word(&word, this);
            these_shapes.shove(cart_pos);
            Ok((these_shapes,this.rotate(diff)))
        },
        crate::tree::WordTypes::Number(word) => {
            let num = &Number::try_from(word.clone())?;
            let cart_pos:Cart = Cart::from(this.clone());
            let mut these_shapes:BShape = num.into();
            these_shapes.shove(cart_pos);
            Ok((these_shapes,this.rotate(diff)))
        },
        Punctuation(word) => {
            todo!()
        }
    }
}

impl TryFrom<&Sentence> for (BShape,f64) {//also return length
    type Error = String;
    fn try_from(sentence:&Sentence) -> Result<Self,Self::Error> {
        let num_words = sentence.get_num_words();

        let (inner_radius ,outer_radius,diff) = get_inner_outer_diff(num_words);

        let mut pos = Polar::new(inner_radius, -TAU/4.0);
        let mut last = pos.clone();

        let mut shapes:Shapes = Vec::new();

        shapes.push(Box::new(Circle::new(Cart::origin(), outer_radius,Some(Thick))));
        shapes.push(Box::new(Circle::new(Cart::origin(), outer_radius-2.0*Thick.val(),Some(Thin))));

        for word in &sentence.words {
            let (new_shapes,new_next) = draw_word(word, &pos, &last, diff)?;
            shapes.push(new_shapes);
            last = pos;
            pos = new_next;
        }
        let length = outer_radius * 1.1 * 2.0;
        let shape = Box::new(ShapeSet::new(shapes, "sentence"));
        return Ok((shape,length));
    }
}