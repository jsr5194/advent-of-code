
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NavCpu {
	pub ship: Position,
	pub waypoint: Position,
}

impl NavCpu {
	pub fn run(&mut self, program:&Vec<Instruction>) {
		for instr in program {
			match instr.action {
				Direction::North   => {
					self.waypoint.go_direction(&instr);
				},
				Direction::South   => {
					self.waypoint.go_direction(&instr);
				},
				Direction::East    => {
					self.waypoint.go_direction(&instr);
				},
				Direction::West    => {
					self.waypoint.go_direction(&instr);
				},
				Direction::Left    => {
					self.waypoint.rotate_heading(&instr)
				},
				Direction::Right   => {
					self.waypoint.rotate_heading(&instr)
				},
				Direction::Forward => {
					for round in 0..instr.value {
						// handle longitude direction
						self.ship.go_direction(&Instruction {
							action: self.waypoint.longitude.direction.clone(),
							value: self.waypoint.longitude.value,
						});

						// handle latitude direction
						self.ship.go_direction(&Instruction{
							action: self.waypoint.latitude.direction.clone(),
							value: self.waypoint.latitude.value,
						});
					}
				},
				_ => panic!("Invalid instruction"),
			}
		}
	}
}


#[derive(Debug, Clone, PartialEq)]
pub struct Latitude {
	pub direction: Direction,
	pub value: usize,
}

impl Default for Latitude {
	fn default() -> Self {
		Latitude {
			direction: Direction::North,
			value: usize::default(),
		}
	}
}

impl Latitude {
	fn invert_direction(&mut self) {
		match self.direction {
			Direction::North  => self.direction = Direction::South,
			Direction::South  => self.direction = Direction::North,
			_ => panic!("could not invert latitude direction"),
		};
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Longitude {
	pub direction: Direction,
	pub value: usize,
}

impl Default for Longitude {
	fn default() -> Self {
		Longitude {
			direction: Direction::East,
			value: usize::default(),
		}
	}
}

impl Longitude {
	fn invert_direction(&mut self) {
		match self.direction {
			Direction::East  => self.direction = Direction::West,
			Direction::West  => self.direction = Direction::East,
			_ => panic!("could not invert longitude direction"),
		};
	}
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Position {
	pub latitude: Latitude,
	pub longitude: Longitude,
}

impl Position {
	fn get_heading(&self) -> Direction {
		match self.latitude.direction {
			Direction::North => {
				match self.longitude.direction {
					Direction::East => return Direction::NorthEast,
					Direction::West => return Direction::NorthWest,
					_ => panic!("invalid North X direction"),
				};
			},
			Direction::South => {
				match self.longitude.direction {
					Direction::East => return Direction::SouthEast,
					Direction::West => return Direction::SouthWest,
					_ => panic!("invalid South X direction"),
				};
			},
			_ => panic!("invalid longitude direction"),
		};
	}

	fn go_direction(&mut self, instr: &Instruction) {
		match instr.action {
			Direction::North   => {
				self.change_latitude(instr);
			},
			Direction::South   => {
				self.change_latitude(instr);
			},
			Direction::East    => {
				self.change_longitude(instr);
			},
			Direction::West    => {
				self.change_longitude(instr);
			},
			_ => panic!("Invalid action for go_direction"),
		}


	}

	fn change_latitude(&mut self, instr: &Instruction) {
		if self.latitude.direction == instr.action {
			self.latitude.value += instr.value;
		} else {
			// need to do an wrapping_sub to handle when we cross the equator
			if self.latitude.value.overflowing_sub(instr.value).1 {
				self.latitude.invert_direction();
				self.latitude.value = instr.value - self.latitude.value;
			} else {
				self.latitude.value = self.latitude.value - instr.value;
			}
		}
	}

	fn change_longitude(&mut self, instr: &Instruction) {
		if self.longitude.direction == instr.action {
			self.longitude.value += instr.value;
		} else {
			// need to do an wrapping_sub to handle when we cross the PM
			if self.longitude.value.overflowing_sub(instr.value).1 {
				self.longitude.invert_direction();
				self.longitude.value = instr.value - self.longitude.value;
			} else {
				self.longitude.value = self.longitude.value - instr.value;
			}
		}
	}

	fn rotate_heading(&mut self, instr: &Instruction) {
		assert!(instr.action == Direction::Left || instr.action == Direction::Right);

		// make sure the rotation is acceptable
		if instr.value % 90 != 0 {
			panic!("Invalid degrees");
		}

		// going left is just 180 degress from the value going right
		let mut normalized_value = instr.value;
		if instr.action == Direction::Left {
			if normalized_value != 180 {
				normalized_value += 180;
			}
		}

		// keep it to only 3 allowed angles
		normalized_value = normalized_value % 360;

		// save off a temporary version of the position 
		let old_position = self.clone();

		// change heading
		match self.get_heading() {
			Direction::North => {
				match normalized_value {
					// Right or Left 90
					90  => {
						self.latitude.value = old_position.longitude.value;
						self.longitude.value = old_position.latitude.value;
						self.longitude.direction = Direction::East;
					},
					// Right or Left 180
					180 => {
						self.latitude.direction = Direction::South;
					},
					// Right or Left 270
					270 => {
						self.latitude.value = old_position.longitude.value;
						self.longitude.value = old_position.latitude.value;
						self.longitude.direction = Direction::West;
					},
					_ => panic!("invalid rotation angle"),
				};
			},
			Direction::South => {
				match normalized_value {
					// Right or Left 90
					90  => {
						self.latitude.value = old_position.longitude.value;
						self.longitude.value = old_position.latitude.value;
						self.longitude.direction = Direction::West;
					},
					// Right or Left 180
					180 => {
						self.latitude.direction = Direction::North;
					},
					// Right or Left 270
					270 => {
						self.latitude.value = old_position.longitude.value;
						self.longitude.value = old_position.latitude.value;
						self.longitude.direction = Direction::East;
					},
					_ => panic!("invalid rotation angle"),
				};
			},
			Direction::East  => {
				match normalized_value {
					// Right or Left 90
					90  => {
						self.latitude.value = old_position.longitude.value;
						self.longitude.value = old_position.latitude.value;
						self.latitude.direction = Direction::South;
					},
					// Right or Left 180
					180 => {
						self.longitude.direction = Direction::West;
					},
					// Right or Left 270
					270 => {
						self.latitude.value = old_position.longitude.value;
						self.longitude.value = old_position.latitude.value;
						self.latitude.direction = Direction::North;
					},
					_ => panic!("invalid rotation angle"),
				};

			},
			Direction::West  => {
				match normalized_value {
					// Right or Left 90
					90  => {
						self.latitude.value = old_position.longitude.value;
						self.longitude.value = old_position.latitude.value;
						self.latitude.direction = Direction::North;
					},
					// Right or Left 180
					180 => {
						self.longitude.direction = Direction::East;
					},
					// Right or Left 270
					270 => {
						self.latitude.value = old_position.longitude.value;
						self.longitude.value = old_position.latitude.value;
						self.latitude.direction = Direction::South;
					},
					_ => panic!("invalid rotation angle"),
				};
			},
			Direction::NorthEast  => {
				match normalized_value {
					// Right 90 Left 270
					90  => {
						self.latitude.value = old_position.longitude.value;
						self.longitude.value = old_position.latitude.value;
						self.latitude.direction = Direction::South;
					},
					// Right 180 Left 180
					180 => {
						self.latitude.direction = Direction::South;
						self.longitude.direction = Direction::West;
					},
					// Right 270 Left 90
					270 => {
						self.latitude.value = old_position.longitude.value;
						self.longitude.value = old_position.latitude.value;
						self.longitude.direction = Direction::West;
					},
					_ => panic!("invalid rotation angle"),
				};
			},
			Direction::NorthWest  => {
				match normalized_value {
					// Right 90 Left 270
					90  => {
						self.latitude.value = old_position.longitude.value;
						self.longitude.value = old_position.latitude.value;
						self.longitude.direction = Direction::East;
					},
					// Right 180 Left 180
					180 => {
						self.latitude.direction = Direction::South;
						self.longitude.direction = Direction::East;
					},
					// Right 270 Left 90
					270 => {
						self.latitude.value = old_position.longitude.value;
						self.longitude.value = old_position.latitude.value;
						self.latitude.direction = Direction::South;
					},
					_ => panic!("invalid rotation angle"),
				};
			},
			Direction::SouthEast  => {
				match normalized_value {
					// Right 90 Left 270
					90  => {
						self.latitude.value = old_position.longitude.value;
						self.longitude.value = old_position.latitude.value;
						self.longitude.direction = Direction::West;
					},
					// Right 180 Left 180
					180 => {
						self.latitude.direction = Direction::North;
						self.longitude.direction = Direction::West;
					},
					// Right 270 Left 90
					270 => {
						self.latitude.value = old_position.longitude.value;
						self.longitude.value = old_position.latitude.value;
						self.latitude.direction = Direction::North;
					},
					_ => panic!("invalid rotation angle"),
				};
			},
			Direction::SouthWest  => {
				match normalized_value {
					// Right 90 Left 270
					90  => {
						self.latitude.value = old_position.longitude.value;
						self.longitude.value = old_position.latitude.value;
						self.latitude.direction = Direction::North;
					},
					// Right 180 Left 180
					180 => {
						self.latitude.direction = Direction::North;
						self.longitude.direction = Direction::East;
					},
					// Right 270 Left 90
					270 => {
						self.latitude.value = old_position.longitude.value;
						self.longitude.value = old_position.latitude.value;
						self.longitude.direction = Direction::East;
					},
					_ => panic!("invalid rotation angle"),
				};
			},
			_ => panic!("invalid heading detected"),
		};
	}
}


#[derive(Debug, Default, Clone, PartialEq)]
pub struct Instruction {
	pub action: Direction,
	pub value: usize,
}


#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
	North,
	South,
	East,
	West,
	NorthEast,
	NorthWest,
	SouthEast,
	SouthWest,
	Left,
	Right,
	Forward,
	Steady,
}

impl Default for Direction {
	fn default() -> Self {
		Direction::Steady
	}
}