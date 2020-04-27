#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Debug)]
struct Relation {
    name: &'static str,
}

impl Relation {
    fn from_name(name: &'static str) -> Relation {
        Relation {
            name,
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Debug)]
struct Object {
    namespace: &'static str,
    object_id: &'static str,
}
#[derive(Clone, PartialEq, PartialOrd, Eq, Debug)]
struct UserSet {
    object: Object,
    relation: Relation,
}
#[derive(Clone, PartialEq, PartialOrd, Eq, Debug)]
enum User {
    Id(u64),
    Set(UserSet),
}
#[derive(Clone, PartialEq, PartialOrd, Eq, Debug)]
struct RelTuple {
    object: Object,
    relation: Relation,
    user: User,
}

struct RelDB {
    rels: Vec<RelTuple>,
}

impl RelDB {
    fn new() -> RelDB {
        RelDB { rels: vec![] }
    }

    fn add(&mut self, rel: RelTuple) {
        self.rels.push(rel);
    }

    fn check(&self, user_set: User, obj: Object, rel: Relation) -> bool {
        let check_rel = RelTuple {
            object: obj,
            user: user_set,
            relation: rel,
        };

        for db_rel in self.rels.iter() {
            if db_rel.user == check_rel.user {
                if *db_rel == check_rel {
                    return true;
                } else {
                    let user = UserSet {
                        object: db_rel.object,
                        relation: db_rel.relation.clone(),
                    };

                    let user = User::Set(user);
                    if self.check(user, obj, rel.clone()) {
                        return true;
                    }

                    //let rewrites = rel.inherited_from.map_or(vec![], |f| f());

                    // for re_rel in rel.inherited_from.iter() {
                    //     if self.check(user, obj, *re_rel) {
                    //         return true;
                    //     }
                    // }
                }
            }
        }

        return false;
    }
}

fn main() {
    let mut db = RelDB::new();

    // define owner relation
    let owner = Relation::from_name("owner");

    // define editor relation

    // define viewer relation
    fn view_rewrite() -> Vec<Relation> {
        vec![Relation::from_name("owner")]
    }

    let viewer = Relation {
        name: "viewer",
    };

    let member = Relation::from_name("member");

    let readme = Object {
        namespace: "doc",
        object_id: "readme",
    };

    let eng_group = Object {
        namespace: "group",
        object_id: "eng",
    };

    // user 10 is  an owner  of doc:readme
    db.add(RelTuple {
        object: readme,
        relation: owner,
        user: User::Id(10),
    });

    // user 3 is a view of doc:readme
    db.add(RelTuple {
        object: readme,
        relation: viewer,
        user: User::Id(3),
    });

    // user 11 is a memeber of group:eng
    db.add(RelTuple {
        object: eng_group,
        relation: member,
        user: User::Id(11),
    });

    // member of group:eng are views of doc:readme
    db.add(RelTuple {
        object: readme,
        relation: viewer,
        user: User::Set(UserSet {
            object: eng_group,
            relation: member,
        }),
    });

    assert!(db.check(User::Id(10), readme, owner));
    assert!(db.check(User::Id(11), eng_group, member));
    assert!(db.check(User::Id(11), readme, viewer));

    // inherited from owner
    assert!(db.check(User::Id(11), readme, viewer));

    assert_eq!(db.check(User::Id(3), readme, viewer), true);
    assert_eq!(db.check(User::Id(3), readme, owner), false);

    println!("done.")
}
