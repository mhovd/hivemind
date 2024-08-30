use crate::solver::{
    event,
    interpolation_method::{self, CarryForward as ProtoCarryForward, Linear as ProtoLinear},
    observation, Bolus as ProtoBolus, Covariate as ProtoCovariate,
    CovariateSegment as ProtoCovariateSegment, Covariates as ProtoCovariates, Event as ProtoEvent,
    Infusion as ProtoInfusion, InterpolationMethod as ProtoInterpolationMethod,
    Observation as ProtoObservation, Occasion as ProtoOccasion, Subject as ProtoSubject,
};
use pharmsol::{
    Bolus, Covariate, CovariateSegment, Covariates, Event, Infusion, InterpolationMethod,
    Observation, Occasion, Subject,
};

impl From<&Subject> for ProtoSubject {
    fn from(subject: &Subject) -> Self {
        ProtoSubject {
            id: subject.id().clone(),
            occasions: subject
                .occasions()
                .into_iter()
                .map(ProtoOccasion::from)
                .collect(),
        }
    }
}

impl From<&Occasion> for ProtoOccasion {
    fn from(occasion: &Occasion) -> Self {
        ProtoOccasion {
            events: occasion
                .events()
                .into_iter()
                .map(ProtoEvent::from)
                .collect(),
            covariates: match occasion.get_covariates() {
                Some(covariates) => Some(ProtoCovariates::from(covariates)),
                None => None,
            },
            index: occasion.index() as u32,
        }
    }
}

impl From<&Event> for ProtoEvent {
    fn from(event: &Event) -> Self {
        match event {
            Event::Bolus(bolus) => ProtoEvent {
                event_type: Some(event::EventType::Bolus(ProtoBolus::from(bolus))),
            },
            Event::Infusion(infusion) => ProtoEvent {
                event_type: Some(event::EventType::Infusion(ProtoInfusion::from(infusion))),
            },
            Event::Observation(observation) => ProtoEvent {
                event_type: Some(event::EventType::Observation(ProtoObservation::from(
                    observation,
                ))),
            },
        }
    }
}

impl From<&Bolus> for ProtoBolus {
    fn from(bolus: &Bolus) -> Self {
        ProtoBolus {
            time: bolus.time(),
            amount: bolus.amount(),
            input: bolus.input() as u32,
        }
    }
}

impl From<&Infusion> for ProtoInfusion {
    fn from(infusion: &Infusion) -> Self {
        ProtoInfusion {
            time: infusion.time(),
            amount: infusion.amount(),
            input: infusion.input() as u32,
            duration: infusion.duration(),
        }
    }
}

impl From<&Observation> for ProtoObservation {
    fn from(observation: &Observation) -> Self {
        ProtoObservation {
            time: observation.time(),
            value: observation.value(),
            outeq: observation.outeq() as u32,
            errorpoly: observation
                .errorpoly()
                .map(|(a, b, c, d)| observation::ErrorPoly { a, b, c, d }),
            ignore: observation.ignore(),
        }
    }
}

impl From<&Covariates> for ProtoCovariates {
    fn from(covariates: &Covariates) -> Self {
        ProtoCovariates {
            covariates: covariates
                .covariates()
                .into_iter()
                .map(|(k, v)| (k, ProtoCovariate::from(v)))
                .collect(),
        }
    }
}

impl From<&Covariate> for ProtoCovariate {
    fn from(covariate: &Covariate) -> Self {
        ProtoCovariate {
            name: covariate.name().to_string(),
            segments: covariate
                .segments()
                .into_iter()
                .map(ProtoCovariateSegment::from)
                .collect(),
        }
    }
}

impl From<&CovariateSegment> for ProtoCovariateSegment {
    fn from(segment: &CovariateSegment) -> Self {
        ProtoCovariateSegment {
            from: segment.from(),
            to: segment.to(),
            method: Some(ProtoInterpolationMethod::from(segment.method())),
        }
    }
}

impl From<&InterpolationMethod> for ProtoInterpolationMethod {
    fn from(method: &InterpolationMethod) -> Self {
        match method {
            InterpolationMethod::Linear { slope, intercept } => ProtoInterpolationMethod {
                method: Some(interpolation_method::Method::Linear(ProtoLinear {
                    slope: *slope,
                    intercept: *intercept,
                })),
            },
            InterpolationMethod::CarryForward { value } => ProtoInterpolationMethod {
                method: Some(interpolation_method::Method::CarryForward(
                    ProtoCarryForward { value: *value },
                )),
            },
        }
    }
}
