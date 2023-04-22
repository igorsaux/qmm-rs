use std::collections::BTreeMap;

use fastrand::Rng;
use qmm_syntax::{
    qmm::*,
    text::formatted_text::{FormattedText, TextElementKind},
};

pub enum PlayerAction {
    DoNothing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StepResult {
    InProgress,
}

#[derive(Debug, Clone)]
pub struct LocationState {
    pub id: LocationId,
    pub description: FormattedText,
}

#[derive(Debug, Clone)]
pub struct JumpState {
    pub id: JumpId,
    pub name: FormattedText,
    pub available: bool,
}

#[derive(Debug, Clone)]
pub struct QuestState {
    pub location: LocationState,
    pub jumps: Vec<JumpState>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum QuestError {
    NoStartingLocation,
}

#[derive(Debug, Clone)]
pub struct QuestPlayer<'q> {
    quest: &'q Quest,
    state: QuestState,
    task_text: FormattedText,
    variables: BTreeMap<String, String>,
    rng: Rng,
}

impl<'q> QuestPlayer<'q> {
    pub fn new(quest: &'q Quest, seed: u64) -> Result<Self, QuestError> {
        let starting_location = quest
            .locations
            .iter()
            .find(|loc| matches!(loc.ty, LocationType::Starting))
            .ok_or(QuestError::NoStartingLocation)?;

        let variables = default_variables();
        let mut jumps = Vec::new();

        for jump in &quest.jumps {
            if jump.from != starting_location.id {
                continue;
            }

            jumps.push(JumpState {
                id: jump.id,
                name: jump.text.clone(),
                available: true,
            })
        }

        let state = QuestState {
            location: LocationState {
                id: starting_location.id,
                description: Self::replace_formatted_text(
                    &variables,
                    starting_location.texts.first().cloned().unwrap(),
                ),
            },
            jumps,
        };

        let task_text = Self::replace_formatted_text(&variables, quest.info.task_text.clone());
        let rng = Rng::with_seed(seed);

        Ok(Self {
            quest,
            state,
            task_text,
            rng,
            variables,
        })
    }

    fn replace_formatted_text(
        variables: &BTreeMap<String, String>,
        mut text: FormattedText,
    ) -> FormattedText {
        for el in &mut text.elements {
            if !matches!(el.kind, TextElementKind::Variable { .. }) {
                continue;
            }

            if let Some(value) = variables.get(&el.value) {
                el.value = value.to_string();
            }
        }

        text
    }

    pub fn task_text(&self) -> &FormattedText {
        &self.task_text
    }

    pub fn step(&mut self, action: PlayerAction) -> StepResult {
        match action {
            PlayerAction::DoNothing => StepResult::InProgress,
        }
    }

    pub fn state(&self) -> &QuestState {
        &self.state
    }

    pub fn quest(&self) -> &Quest {
        self.quest
    }
}

fn default_variables() -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();

    map.insert("<ToStar>".to_string(), "Процион".to_string());
    map.insert("<ToPlanet>".to_string(), "Боннасис".to_string());
    map.insert("<FromStar>".to_string(), "Солнечная".to_string());
    map.insert("<FromPlanet>".to_string(), "Земля".to_string());
    map.insert("<Ranger>".to_string(), "Греф".to_string());
    map.insert("<Date>".to_string(), "15 Марта 3300".to_string());
    map.insert("<Day>".to_string(), "15 Марта".to_string());
    map.insert("<Money>".to_string(), "10000".to_string());

    map
}
