#![cfg(feature = "nova")]

use std::io::Read;
use crate::vm::{ChimeraVM, Value};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
/// Enum for `Arcana`.
pub enum Arcana {
    /// TheFool
    TheFool = 0,
    /// TheMagician
    TheMagician = 1,
    /// TheHighPriestess
    TheHighPriestess = 2,
    /// TheEmpress
    TheEmpress = 3,
    /// TheEmperor
    TheEmperor = 4,
    /// TheHierophant
    TheHierophant = 5,
    /// TheLovers
    TheLovers = 6,
    /// TheChariot
    TheChariot = 7,
    /// Strength
    Strength = 8,
    /// TheHermit
    TheHermit = 9,
    /// WheelOfFortune
    WheelOfFortune = 10,
    /// Justice
    Justice = 11,
    /// TheHangedMan
    TheHangedMan = 12,
    /// Death
    Death = 13,
    /// Temperance
    Temperance = 14,
    /// TheDevil
    TheDevil = 15,
    /// TheTower
    TheTower = 16,
    /// TheStar
    TheStar = 17,
    /// TheMoon
    TheMoon = 18,
    /// TheSun
    TheSun = 19,
    /// Judgement
    Judgement = 20,
    /// TheWorld
    TheWorld = 21,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
/// Represents a `FateState`.
pub struct FateState {
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    /// The `active_arcana` field.
    pub active_arcana: Option<Arcana>,
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    /// The `duration` field.
    pub duration: usize,
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    /// The `deck` field.
    pub deck: Vec<Arcana>,
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    /// The `discard` field.
    pub discard: Vec<Arcana>,
}

impl Default for FateState {
    fn default() -> Self {
        Self::new()
    }
}

impl FateState {
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    pub fn new() -> Self {
        let mut deck = Vec::new();
        // Initialize with all Major Arcana
        for i in 0..=21 {
            deck.push(match i {
                0 => Arcana::TheFool,
                1 => Arcana::TheMagician,
                2 => Arcana::TheHighPriestess,
                3 => Arcana::TheEmpress,
                4 => Arcana::TheEmperor,
                5 => Arcana::TheHierophant,
                6 => Arcana::TheLovers,
                7 => Arcana::TheChariot,
                8 => Arcana::Strength,
                9 => Arcana::TheHermit,
                10 => Arcana::WheelOfFortune,
                11 => Arcana::Justice,
                12 => Arcana::TheHangedMan,
                13 => Arcana::Death,
                14 => Arcana::Temperance,
                15 => Arcana::TheDevil,
                16 => Arcana::TheTower,
                17 => Arcana::TheStar,
                18 => Arcana::TheMoon,
                19 => Arcana::TheSun,
                20 => Arcana::Judgement,
                21 => Arcana::TheWorld,
                _ => Arcana::TheFool,
            });
        }

        // Simple Fisher-Yates shuffle (done by exec_shuffle later)
        // For now, deterministic or ordered is fine as default.

        Self {
            active_arcana: None,
            duration: 0,
            deck,
            discard: Vec::new(),
        }
    }
}

/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
/// Performs the `exec_draw` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_draw
/// ```
pub fn exec_draw(vm: &mut ChimeraVM) {
    // Draw: Pop from deck, set active, set duration (100 ticks).
    // Push card ID to stack.
    if vm.fate.deck.is_empty() {
        vm.stack.push(Value::Int(-1)); // Deck empty
        vm.output.push("FATE: Deck empty".to_string());
        return;
    }

    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..vm.fate.deck.len());
    let card = vm.fate.deck.remove(idx);

    // If there was an active card, move to discard
    if let Some(old_card) = vm.fate.active_arcana.take() {
        vm.fate.discard.push(old_card);
    }

    vm.stack.push(Value::Int(card.clone() as i64));
    vm.output.push(format!("FATE: Drawn {:?}", card));

    vm.fate.active_arcana = Some(card);
    vm.fate.duration = 100;
}

/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
/// Performs the `exec_shuffle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_shuffle
/// ```
pub fn exec_shuffle(vm: &mut ChimeraVM) {
    // Move discard to deck
    vm.fate.deck.append(&mut vm.fate.discard);
    // Shuffle implies just making the deck ready for random draws.
    // Since draw uses random index, we don't strictly need to shuffle the vec itself,
    // but clearing discard is the key.
    vm.output.push("FATE: Shuffled deck".to_string());
}

/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
/// Performs the `exec_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_fate
/// ```
pub fn exec_fate(vm: &mut ChimeraVM) {
    // Pushes active card ID to stack, or -1 if none.
    if let Some(card) = &vm.fate.active_arcana {
        vm.stack.push(Value::Int(card.clone() as i64));
    } else {
        vm.stack.push(Value::Int(-1));
    }
}

/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
/// Performs the `process_fate` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of process_fate
/// ```
pub fn process_fate(vm: &mut ChimeraVM) {
    if vm.fate.duration > 0 {
        vm.fate.duration -= 1;
        if vm.fate.duration == 0 {
            if let Some(card) = vm.fate.active_arcana.take() {
                vm.output.push(format!("FATE: {:?} expired", card));
                vm.fate.discard.push(card);
            }
        }
    }

    if let Some(card) = &vm.fate.active_arcana {
        match card {
            Arcana::TheMagician => {
                // Creation: +Energy
                vm.energy = vm.energy.saturating_add(2);
            }
            Arcana::TheHighPriestess => {
                // Intuition: Reveal hidden (GRead bonus? For now, nothing passive)
            }
            Arcana::TheEmpress => {
                // Nature: Growth (Botany speed up?)
                // Simple: Reduce entropy
                let (cy, cx) = vm.context_loc;
                if vm.entropy_grid[cy][cx] > 0 {
                    vm.entropy_grid[cy][cx] -= 1;
                }
            }
            Arcana::TheEmperor => {
                // Authority: Order established
                vm.output
                    .push("FATE: The Emperor demands order.".to_string());
            }
            Arcana::TheHierophant => {
                // Tradition: Faith
                vm.egregore.faith = vm.egregore.faith.saturating_add(1);
            }
            Arcana::TheLovers => {
                // Union: Entanglement chance?
            }
            Arcana::TheChariot => {
                // Willpower: Move towards goal?
            }
            Arcana::Strength => {
                // Courage: Resist damage?
            }
            Arcana::TheHermit => {
                // Introspection: Lower Chaos
                if vm.chaos_mode {
                    // Suppress chaos temporarily
                    // (Implementation detail: chaos_mode check happens later in step(), so we can't easily suppress it here without a flag)
                }
            }
            Arcana::WheelOfFortune => {
                // Luck: Random Energy Flux
                let mut rng = rand::thread_rng();
                let flux = rng.gen_range(-5..=5);
                vm.energy = vm.energy.saturating_add(flux);
            }
            Arcana::Justice => {
                // Balance: Equalize stacks?
            }
            Arcana::TheHangedMan => {
                // Sacrifice: Lose energy, gain Insight (Stack push?)
                vm.energy = vm.energy.saturating_sub(1);
            }
            Arcana::Death => {
                // Transformation: Clear Waste
                let (cy, cx) = vm.context_loc;
                vm.waste_grid[cy][cx] = 0;
            }
            Arcana::Temperance => {
                // Moderation: Heat reduction (not implemented)
            }
            Arcana::TheDevil => {
                // Addiction: Gain energy but increase Entropy
                vm.energy = vm.energy.saturating_add(5);
                let (cy, cx) = vm.context_loc;
                vm.entropy_grid[cy][cx] = vm.entropy_grid[cy][cx].saturating_add(2);
            }
            Arcana::TheTower => {
                // Catastrophe: Random damage
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.1) {
                    let (cy, cx) = vm.context_loc;
                    vm.grid[cy][cx] = Value::Int(0);
                    vm.output.push("FATE: The Tower strikes!".to_string());
                }
            }
            Arcana::TheStar => {
                // Hope: Heal if low energy
                if vm.energy < 20 {
                    vm.energy += 5;
                }
            }
            Arcana::TheMoon => {
                // Illusion: Increase Chaos
                // (Handled by chaos check in step, maybe force mutate?)
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.2) {
                    vm.mutate();
                }
            }
            Arcana::TheSun => {
                // Joy: Light
                let (cy, cx) = vm.context_loc;
                vm.light_grid[cy][cx] = 100;
            }
            Arcana::Judgement => {
                // Absolution: Clear all debuffs
                if !vm.buffs.is_empty() {
                    vm.buffs.clear();
                    vm.output.push("FATE: Judgement clears buffs".to_string());
                }
            }
            Arcana::TheWorld => {
                // Completion: Score multiplier?
            }
            Arcana::TheFool => {
                // Beginnings: Random jump?
            }
        }
    }
}
