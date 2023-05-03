pub mod humanoid {
    use crate::creature::body::body::*;
    use crate::creature::organs::organs::*;
    use crate::creature::creature::*;


    const LIMB_SIZE: i32 = 400;
    const HEAD_SIZE: i32 = 300;
    const BODY_SIZE: i32 = 2000;

    pub fn humanoid() -> Creature {
        return Creature {
            species: String::from("Human"),
            health_stats: base_health_stats(),
            body: BodyPart {
                name: String::from("Body"),
                tags: vec![],
                statuses: vec![],
                internal: vec![
                    hearts(1, BODY_SIZE),
                    spine(BODY_SIZE),
                    lungs(2, BODY_SIZE),
                ].concat(),
                children: vec![
                    vec![
                        BodyPart {
                            name: String::from("Head"),
                            tags: vec![],
                            statuses: vec![],
                            internal: vec![
                                eyes(2, HEAD_SIZE),
                                ears(2, HEAD_SIZE),
                                nose(HEAD_SIZE),
                            ].concat(),
                            children: vec![],
                            size: HEAD_SIZE
                        },
                        BodyPart {
                            name: String::from("Left Arm"),
                            tags: vec![BodyPartTag::left],
                            statuses: vec![],
                            internal: vec![],
                            children: vec![],
                            size: LIMB_SIZE
                        },
                        BodyPart {
                            name: String::from("Right Arm"),
                            tags: vec![BodyPartTag::right],
                            statuses: vec![],
                            internal: vec![],
                            children: vec![],
                            size: LIMB_SIZE
                        },
                        BodyPart {
                            name: String::from("Left Leg"),
                            tags: vec![BodyPartTag::left],
                            statuses: vec![],
                            internal: vec![],
                            children: vec![],
                            size: LIMB_SIZE
                        },
                        BodyPart {
                            name: String::from("Right Leg"),
                            tags: vec![BodyPartTag::right],
                            statuses: vec![],
                            internal: vec![],
                            children: vec![],
                            size: LIMB_SIZE
                        }
                    ]
                ].concat(),
                size: BODY_SIZE
            }
        }
    }
}