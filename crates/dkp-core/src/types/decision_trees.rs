use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionTreesFile {
    pub trees: Vec<DecisionTree>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionTree {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub root: TreeNode,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audience: Vec<String>,
}

/// A node is either a decision point (has `question` + `branches`) or a
/// terminal answer (has `answer`). The actual format uses these field names
/// rather than a discriminant tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TreeNode {
    Decision {
        question: String,
        #[serde(default)]
        branches: Vec<Branch>,
    },
    Terminal {
        answer: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub condition: String,
    pub next: Box<TreeNode>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_tree_round_trips() {
        let file = DecisionTreesFile {
            trees: vec![DecisionTree {
                id: "t1".to_string(),
                title: None,
                description: None,
                root: TreeNode::Terminal {
                    answer: "Do X".to_string(),
                },
                tags: vec![],
                source_ref: None,
                audience: vec![],
            }],
        };
        let json = serde_json::to_string(&file).unwrap();
        let back: DecisionTreesFile = serde_json::from_str(&json).unwrap();
        match &back.trees[0].root {
            TreeNode::Terminal { answer } => assert_eq!(answer, "Do X"),
            TreeNode::Decision { .. } => panic!("expected terminal node"),
        }
    }

    #[test]
    fn decision_tree_with_branches_round_trips() {
        let root = TreeNode::Decision {
            question: "Is it urgent?".to_string(),
            branches: vec![Branch {
                condition: "yes".to_string(),
                next: Box::new(TreeNode::Terminal {
                    answer: "Escalate".to_string(),
                }),
            }],
        };
        let json = serde_json::to_string(&root).unwrap();
        let back: TreeNode = serde_json::from_str(&json).unwrap();
        match back {
            TreeNode::Decision { question, branches } => {
                assert_eq!(question, "Is it urgent?");
                assert_eq!(branches.len(), 1);
                assert_eq!(branches[0].condition, "yes");
            }
            TreeNode::Terminal { .. } => panic!("expected decision node"),
        }
    }
}
