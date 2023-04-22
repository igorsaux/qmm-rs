pub mod qmm;
pub mod text;

#[cfg(test)]
mod qmm_tests {
    use std::fs;

    use crate::{
        qmm::*,
        text::{
            formatted_text::{FormattedText, TextElement, TextElementKind},
            formula::Formula,
        },
    };

    pub fn quest_data() -> Vec<u8> {
        fs::read("test.qmm").unwrap()
    }

    #[test]
    pub fn parse_header() {
        let data = quest_data();
        let quest = parse_qmm(&data).unwrap();
        let header = quest.header;

        assert_eq!(
            header,
            Header {
                version: Version::Qmm6,
                giver_race: Race::Gaal,
                completion_condition: CompletionCondition::Immediately,
                quest_planet_type: PlanetType::Populated(Race::Peleng),
                player_status: PlayerStatus::all(),
                player_race: Race::all(),
                relation_change: 5,
                default_jumps_limit: JumpsLimit::Limit(1),
                difficult: 70,
                parameters_count: 24
            }
        );
    }

    #[test]
    pub fn parse_params() {
        let data = quest_data();
        let quest = parse_qmm(&data).unwrap();
        let params = quest.parameters;

        assert_eq!(params.len(), quest.header.parameters_count);
        assert_eq!(params.len(), params.capacity());
        assert_eq!(
            params[0],
            Parameter {
                min_value: 0,
                max_value: 20,
                ty: ParameterType::Death,
                show_when_zero: true,
                critical_value: CriticalValue::Min,
                is_active: true,
                is_money: false,
                name: "Здоровье".to_string(),
                formatted_range_lines: vec![
                    FormattedRangeLine {
                        from: 0,
                        to: 0,
                        value: "Вы мертвы".to_string()
                    },
                    FormattedRangeLine {
                        from: 1,
                        to: 20,
                        value: "Ваше здоровье: {[p1]*5}%".to_string()
                    }
                ],
                critical_text: "Внезапно вы почувствовали резкую боль в груди. От большой потери крови у вас закружилась голова, и вы упали на землю. Удар головой обо что-то твердое стал последним ощущением в вашей жизни...".to_string(),
                image: "Diamond_01".to_string(),
                sound: "".to_string(),
                track: "".to_string(),
                starting_value: "[20]".to_string()
            }
        );

        assert_eq!(params[0].name.len(), params[0].name.capacity());
    }

    #[test]
    pub fn parse_string_replacements() {
        let data = quest_data();
        let quest = parse_qmm(&data).unwrap();
        let replacements = quest.string_replacements;

        assert_eq!(
            replacements,
            StringReplacements {
                to_star: "Бред Сумасшедшего".to_string(),
                to_planet: "Дурдом".to_string(),
                from_planet: "Планета дураков".to_string(),
                from_star: "Главный Дурдом".to_string(),
                ranger: "Даун".to_string(),
            }
        )
    }

    #[test]
    pub fn parse_info() {
        let data = quest_data();
        let quest = parse_qmm(&data).unwrap();
        let info = quest.info;

        assert_eq!(info, Info {
            locations_count: 134,
            jumps_count: 351,
            success_text: FormattedText { elements: vec![TextElement {
                kind: TextElementKind::Text,
                value: "Поздравляем вас, ".to_string()
            }, TextElement {
                kind: TextElementKind::Variable { name: "Ranger".to_string() },
                value: "<Ranger>".to_string()
            }, TextElement {
                kind: TextElementKind::Text,
                value: "! Вы сумели уничтожить самого опасного бандита системы ".to_string()
            }, TextElement {
                kind: TextElementKind::Variable { name: "ToStar".to_string() },
                value: "<ToStar>".to_string()
            }, TextElement {
                kind: TextElementKind::Text,
                value: ", а также раскрыть весьма разветвленную сеть наркоторговцев. Благодаря вам миллионы гаальцев смогут почувствовать себя свободными от наркотиков. Эта сумма в ".to_string()
            }, TextElement {
                kind: TextElementKind::Variable { name: "Money".to_string() },
                value: "<Money>".to_string()
            }, TextElement {
                kind: TextElementKind::Text,
                value: " cr теперь по праву ваша.".to_string()
            }] },
            task_text: FormattedText {
                elements: vec![
                    TextElement {
                        kind: TextElementKind::Text,
                        value: "У нас есть очень рискованное, но зато и высокооплачиваемое задание для смелого и решительного рейнджера. Вы должны прибыть на планету ".to_string()
                    },
                    TextElement {
                        kind: TextElementKind::Variable {
                            name: "ToPlanet".to_string()
                        },
                        value: "<ToPlanet>".to_string()
                    },
                    TextElement {
                        kind: TextElementKind::Text,
                        value: " системы ".to_string()
                    },
                    TextElement {
                        kind: TextElementKind::Variable { name: "ToStar".to_string() },
                        value: "<ToStar>".to_string()
                    },
                    TextElement {
                        kind: TextElementKind::Text,
                        value: " не позднее ".to_string()
                    },
                    TextElement {
                        kind: TextElementKind::Variable { name: "Date".to_string() },
                        value: "<Date>".to_string()
                    },
                    TextElement {
                        kind: TextElementKind::Text,
                        value: " и поступить в распоряжение нашего агента, адрес которого мы вам дадим. Дальнейшие инструкции получите на месте. ".to_string()
                    },
                    TextElement {
                        kind: TextElementKind::NewLine,
                        value: "\r\n".to_string()
                    },
                    TextElement {
                        kind: TextElementKind::NewLine,
                        value: "\r\n".to_string()
                    },
                    TextElement {
                        kind: TextElementKind::Text,
                        value: "Сразу же хотим предупредить вас о чрезвычайно большой опасности этого задания (именно по этим соображениям мы вынуждены использовать наемника - ведь гаальские законы запрещают нам использовать своих сотрудников для выполнения заданий, вероятность смертельного исхода которых превышает 50%). ".to_string()
                    },
                    TextElement {
                        kind: TextElementKind::NewLine,
                        value: "\r\n".to_string()
                    },
                    TextElement {
                        kind: TextElementKind::NewLine,
                        value: "\r\n".to_string()
                    },
                    TextElement {
                        kind: TextElementKind::Text,
                        value: "Однако есть и положительные моменты. В случае успеха вам, помимо награды в ".to_string()
                    },
                    TextElement {
                        kind: TextElementKind::Variable { name: "Money".to_string() },
                        value: "<Money>".to_string()
                    },
                    TextElement {
                        kind: TextElementKind::Text,
                        value: " cr, которую вы получите по возвращении на нашу планету, полагается еще и премия в 100.000 cr непосредственно на планете ".to_string()
                    },
                    TextElement {
                        kind: TextElementKind::Variable { name: "ToPlanet".to_string() },
                        value: "<ToPlanet>".to_string()
                    },
                    TextElement {
                        kind: TextElementKind::Text,
                        value: "; из них 10.000 cr вы получите в качестве аванса сразу же по прибытии в пункт назначения.".to_string()
                    },
                    TextElement {
                        kind: TextElementKind::NewLine,
                        value: "\r\n".to_string()
                    },
                    TextElement {
                        kind: TextElementKind::NewLine,
                        value: "\r\n".to_string()
                    },
                    TextElement {
                        kind: TextElementKind::Text,
                        value: "Итак, вы согласны?".to_string()
                    },
                ]
            }
        });
        assert_eq!(info.locations_count as usize, quest.locations.len());
    }

    #[test]
    pub fn parse_locations() {
        let data = quest_data();
        let quest = parse_qmm(&data).unwrap();
        let locations = quest.locations;

        assert_eq!(locations.len(), locations.capacity());
        assert_eq!(
            locations[0],
            Location {
                do_pass_day: false,
                id: LocationId(1),
                max_visits: MaxVisits::Infinite,
                ty: LocationType::Starting,
                parameter_changes: locations[0].parameter_changes.clone(),
                texts: vec![FormattedText {
                    elements: vec![
                        TextElement {
                            kind: TextElementKind::Text,
                            value: "Прибыв на планету ".to_string()
                        },
                        TextElement {
                            kind: TextElementKind::Variable {
                                name: "ToPlanet".to_string()
                            },
                            value: "<ToPlanet>".to_string()
                        }, TextElement {
                            kind: TextElementKind::Text,
                            value: " и пройдя таможенные формальности, вы первым делом перевели в ближайшем банкомате ".to_string()
                        }, TextElement {
                            kind: TextElementKind::Selection { text: "10.000".to_string() },
                            value: "<clr>10.000<clrEnd>".to_string()
                        }, TextElement {
                            kind: TextElementKind::Text,
                            value: " cr на свою карточку ".to_string()
                        }, TextElement {
                            kind: TextElementKind::Selection { text: "Galactic Express".to_string() },
                            value: "<clr>Galactic Express<clrEnd>".to_string()
                        }, TextElement {
                            kind: TextElementKind::Text,
                            value: " (вы специально завели себе кредитную карточку, чтобы снизить вероятность ограбления на этой опасной пеленгской планете). Ну что, теперь можно и домой? Заманчивая идея, но с гаальскими спецслужбами лучше не ссориться - еще заморозят счет, чего доброго. Печально вздохнув, вы пошли по указанному вам адресу на встречу с гаальским агентом.".to_string()
                        }, TextElement {
                            kind: TextElementKind::NewLine,
                            value: "\r\n".to_string()
                        }, TextElement {
                            kind: TextElementKind::NewLine,
                            value: "\r\n".to_string()
                        }, TextElement {
                            kind: TextElementKind::Text,
                            value: "Планета ".to_string()
                        }, TextElement {
                            kind: TextElementKind::Variable { name: "ToPlanet".to_string() },
                            value: "<ToPlanet>".to_string()
                        }, TextElement {
                            kind: TextElementKind::Text,
                            value: " показалась вам грязной и вонючей помойкой. Впрочем, вас это не сильно впечатлило. Вы вышли из космопорта, бывшего единственным чистым и опрятным зданием в округе (поскольку этого требовали законы содружества), и направились к высокому дому, в одной из квартир которого вас должен был ждать агент гаальских спецслужб. Поднявшись на тридцать восьмой этаж, вы подошли к нужной вам квартире. К вашему удивлению дверь оказалась не заперта. Вы осторожно вошли внутрь и тут же получили резкий удар по голове. От такой неожиданности вы потеряли сознание и провалились в черноту...".to_string()
                        }
                    ]
                }],
                media: vec![Media {
                    image: "Newflora_01".to_string(),
                    sound: "".to_string(),
                    track: "".to_string()
                }],
                select_type: LocationSelectType::ByOrder
            }
        );
        assert_eq!(
            locations[0].parameter_changes[0],
            ParameterChange {
                parameter_id: 1,
                show_type: ParameterShowType::Hide,
                change_type: ParameterChangeType::Sum,
                formula: Formula::default(),
                critical_text: "".to_string(),
                media: Media {
                    image: "".to_string(),
                    sound: "".to_string(),
                    track: "".to_string(),
                },
            }
        )
    }

    #[test]
    pub fn parse_jumps() {
        let data = quest_data();
        let quest = parse_qmm(&data).unwrap();
        let jumps = quest.jumps;

        assert_eq!(jumps.len(), jumps.capacity());
        assert_eq!(jumps.len(), quest.info.jumps_count as usize);
        assert_eq!(
            jumps[0],
            Jump {
                priority: 1.0,
                do_pass_day: false,
                id: JumpId(2),
                from: LocationId(1),
                to: LocationId(2),
                show_always: false,
                max_visits: MaxVisits::Infinite,
                show_order: 5,
                parameters_conditions: vec![],
                parameter_changes: vec![],
                formula: Formula::default(),
                text: FormattedText {
                    elements: vec![TextElement {
                        kind: TextElementKind::Text,
                        value: "Очнуться".to_string()
                    }]
                },
                description: FormattedText {
                    elements: Vec::new()
                },
                media: Media {
                    image: "".to_string(),
                    sound: "".to_string(),
                    track: "".to_string(),
                },
            }
        )
    }
}
