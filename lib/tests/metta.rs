use hyperon::common::shared::Shared;
use hyperon::space::grounding::GroundingSpace;
use hyperon::metta::text::*;
use hyperon::metta::runner::Metta;

#[test]
fn test_reduce_higher_order() {
    let program = "
        ; Curried plus
        (: plus (-> Number (-> Number Number)))
        (= ((plus $x) $y) (+ $x $y))
        ; Define inc as partial evaluation of plus
        (: inc (-> (-> Number Number)))
        (= (inc) (plus 1))

        !(assertEqualToResult ((inc) 2) (3))
    ";
    let metta = Metta::new(Shared::new(GroundingSpace::new()), Shared::new(Tokenizer::new()));

    let result = metta.run(&mut SExprParser::new(program));

    assert_eq!(result, Ok(vec![vec![]]));
}