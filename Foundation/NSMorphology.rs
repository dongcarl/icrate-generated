//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSGrammaticalGender {
        NSGrammaticalGenderNotSet = 0,
        NSGrammaticalGenderFeminine = 1,
        NSGrammaticalGenderMasculine = 2,
        NSGrammaticalGenderNeuter = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSGrammaticalPartOfSpeech {
        NSGrammaticalPartOfSpeechNotSet = 0,
        NSGrammaticalPartOfSpeechDeterminer = 1,
        NSGrammaticalPartOfSpeechPronoun = 2,
        NSGrammaticalPartOfSpeechLetter = 3,
        NSGrammaticalPartOfSpeechAdverb = 4,
        NSGrammaticalPartOfSpeechParticle = 5,
        NSGrammaticalPartOfSpeechAdjective = 6,
        NSGrammaticalPartOfSpeechAdposition = 7,
        NSGrammaticalPartOfSpeechVerb = 8,
        NSGrammaticalPartOfSpeechNoun = 9,
        NSGrammaticalPartOfSpeechConjunction = 10,
        NSGrammaticalPartOfSpeechNumeral = 11,
        NSGrammaticalPartOfSpeechInterjection = 12,
        NSGrammaticalPartOfSpeechPreposition = 13,
        NSGrammaticalPartOfSpeechAbbreviation = 14,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSGrammaticalNumber {
        NSGrammaticalNumberNotSet = 0,
        NSGrammaticalNumberSingular = 1,
        NSGrammaticalNumberZero = 2,
        NSGrammaticalNumberPlural = 3,
        NSGrammaticalNumberPluralTwo = 4,
        NSGrammaticalNumberPluralFew = 5,
        NSGrammaticalNumberPluralMany = 6,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSGrammaticalCase {
        NSGrammaticalCaseNotSet = 0,
        NSGrammaticalCaseNominative = 1,
        NSGrammaticalCaseAccusative = 2,
        NSGrammaticalCaseDative = 3,
        NSGrammaticalCaseGenitive = 4,
        NSGrammaticalCasePrepositional = 5,
        NSGrammaticalCaseAblative = 6,
        NSGrammaticalCaseAdessive = 7,
        NSGrammaticalCaseAllative = 8,
        NSGrammaticalCaseElative = 9,
        NSGrammaticalCaseIllative = 10,
        NSGrammaticalCaseEssive = 11,
        NSGrammaticalCaseInessive = 12,
        NSGrammaticalCaseLocative = 13,
        NSGrammaticalCaseTranslative = 14,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSGrammaticalPronounType {
        NSGrammaticalPronounTypeNotSet = 0,
        NSGrammaticalPronounTypePersonal = 1,
        NSGrammaticalPronounTypeReflexive = 2,
        NSGrammaticalPronounTypePossessive = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSGrammaticalPerson {
        NSGrammaticalPersonNotSet = 0,
        NSGrammaticalPersonFirst = 1,
        NSGrammaticalPersonSecond = 2,
        NSGrammaticalPersonThird = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSGrammaticalDetermination {
        NSGrammaticalDeterminationNotSet = 0,
        NSGrammaticalDeterminationIndependent = 1,
        NSGrammaticalDeterminationDependent = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSGrammaticalDefiniteness {
        NSGrammaticalDefinitenessNotSet = 0,
        NSGrammaticalDefinitenessIndefinite = 1,
        NSGrammaticalDefinitenessDefinite = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSMorphology")]
    pub struct NSMorphology;

    #[cfg(feature = "Foundation_NSMorphology")]
    unsafe impl ClassType for NSMorphology {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSMorphology")]
unsafe impl NSCoding for NSMorphology {}

#[cfg(feature = "Foundation_NSMorphology")]
unsafe impl NSCopying for NSMorphology {}

#[cfg(feature = "Foundation_NSMorphology")]
unsafe impl NSObjectProtocol for NSMorphology {}

#[cfg(feature = "Foundation_NSMorphology")]
unsafe impl NSSecureCoding for NSMorphology {}

extern_methods!(
    #[cfg(feature = "Foundation_NSMorphology")]
    unsafe impl NSMorphology {
        #[method(grammaticalGender)]
        pub unsafe fn grammaticalGender(&self) -> NSGrammaticalGender;

        #[method(setGrammaticalGender:)]
        pub unsafe fn setGrammaticalGender(&self, grammatical_gender: NSGrammaticalGender);

        #[method(partOfSpeech)]
        pub unsafe fn partOfSpeech(&self) -> NSGrammaticalPartOfSpeech;

        #[method(setPartOfSpeech:)]
        pub unsafe fn setPartOfSpeech(&self, part_of_speech: NSGrammaticalPartOfSpeech);

        #[method(number)]
        pub unsafe fn number(&self) -> NSGrammaticalNumber;

        #[method(setNumber:)]
        pub unsafe fn setNumber(&self, number: NSGrammaticalNumber);

        #[method(grammaticalCase)]
        pub unsafe fn grammaticalCase(&self) -> NSGrammaticalCase;

        #[method(setGrammaticalCase:)]
        pub unsafe fn setGrammaticalCase(&self, grammatical_case: NSGrammaticalCase);

        #[method(determination)]
        pub unsafe fn determination(&self) -> NSGrammaticalDetermination;

        #[method(setDetermination:)]
        pub unsafe fn setDetermination(&self, determination: NSGrammaticalDetermination);

        #[method(grammaticalPerson)]
        pub unsafe fn grammaticalPerson(&self) -> NSGrammaticalPerson;

        #[method(setGrammaticalPerson:)]
        pub unsafe fn setGrammaticalPerson(&self, grammatical_person: NSGrammaticalPerson);

        #[method(pronounType)]
        pub unsafe fn pronounType(&self) -> NSGrammaticalPronounType;

        #[method(setPronounType:)]
        pub unsafe fn setPronounType(&self, pronoun_type: NSGrammaticalPronounType);

        #[method(definiteness)]
        pub unsafe fn definiteness(&self) -> NSGrammaticalDefiniteness;

        #[method(setDefiniteness:)]
        pub unsafe fn setDefiniteness(&self, definiteness: NSGrammaticalDefiniteness);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSMorphology")]
    unsafe impl NSMorphology {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSMorphologyPronoun")]
    pub struct NSMorphologyPronoun;

    #[cfg(feature = "Foundation_NSMorphologyPronoun")]
    unsafe impl ClassType for NSMorphologyPronoun {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSMorphologyPronoun")]
unsafe impl NSCoding for NSMorphologyPronoun {}

#[cfg(feature = "Foundation_NSMorphologyPronoun")]
unsafe impl NSCopying for NSMorphologyPronoun {}

#[cfg(feature = "Foundation_NSMorphologyPronoun")]
unsafe impl NSObjectProtocol for NSMorphologyPronoun {}

#[cfg(feature = "Foundation_NSMorphologyPronoun")]
unsafe impl NSSecureCoding for NSMorphologyPronoun {}

extern_methods!(
    #[cfg(feature = "Foundation_NSMorphologyPronoun")]
    unsafe impl NSMorphologyPronoun {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSMorphology", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithPronoun:morphology:dependentMorphology:)]
        pub unsafe fn initWithPronoun_morphology_dependentMorphology(
            this: Allocated<Self>,
            pronoun: &NSString,
            morphology: &NSMorphology,
            dependent_morphology: Option<&NSMorphology>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other pronoun)]
        pub unsafe fn pronoun(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSMorphology")]
        #[method_id(@__retain_semantics Other morphology)]
        pub unsafe fn morphology(&self) -> Id<NSMorphology>;

        #[cfg(feature = "Foundation_NSMorphology")]
        #[method_id(@__retain_semantics Other dependentMorphology)]
        pub unsafe fn dependentMorphology(&self) -> Option<Id<NSMorphology>>;
    }
);

extern_methods!(
    /// NSCustomPronouns
    #[cfg(feature = "Foundation_NSMorphology")]
    unsafe impl NSMorphology {
        #[cfg(all(
            feature = "Foundation_NSMorphologyCustomPronoun",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "Use NSTermOfAddress instead"]
        #[method_id(@__retain_semantics Other customPronounForLanguage:)]
        pub unsafe fn customPronounForLanguage(
            &self,
            language: &NSString,
        ) -> Option<Id<NSMorphologyCustomPronoun>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSMorphologyCustomPronoun",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "Use NSTermOfAddress instead"]
        #[method(setCustomPronoun:forLanguage:error:_)]
        pub unsafe fn setCustomPronoun_forLanguage_error(
            &self,
            features: Option<&NSMorphologyCustomPronoun>,
            language: &NSString,
        ) -> Result<(), Id<NSError>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSMorphologyCustomPronoun")]
    #[deprecated = "Use NSTermOfAddress instead"]
    pub struct NSMorphologyCustomPronoun;

    #[cfg(feature = "Foundation_NSMorphologyCustomPronoun")]
    unsafe impl ClassType for NSMorphologyCustomPronoun {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSMorphologyCustomPronoun")]
unsafe impl NSCoding for NSMorphologyCustomPronoun {}

#[cfg(feature = "Foundation_NSMorphologyCustomPronoun")]
unsafe impl NSCopying for NSMorphologyCustomPronoun {}

#[cfg(feature = "Foundation_NSMorphologyCustomPronoun")]
unsafe impl NSObjectProtocol for NSMorphologyCustomPronoun {}

#[cfg(feature = "Foundation_NSMorphologyCustomPronoun")]
unsafe impl NSSecureCoding for NSMorphologyCustomPronoun {}

extern_methods!(
    #[cfg(feature = "Foundation_NSMorphologyCustomPronoun")]
    unsafe impl NSMorphologyCustomPronoun {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSTermOfAddress instead"]
        #[method(isSupportedForLanguage:)]
        pub unsafe fn isSupportedForLanguage(language: &NSString) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated = "Use NSTermOfAddress instead"]
        #[method_id(@__retain_semantics Other requiredKeysForLanguage:)]
        pub unsafe fn requiredKeysForLanguage(language: &NSString) -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSTermOfAddress instead"]
        #[method_id(@__retain_semantics Other subjectForm)]
        pub unsafe fn subjectForm(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSTermOfAddress instead"]
        #[method(setSubjectForm:)]
        pub unsafe fn setSubjectForm(&self, subject_form: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSTermOfAddress instead"]
        #[method_id(@__retain_semantics Other objectForm)]
        pub unsafe fn objectForm(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSTermOfAddress instead"]
        #[method(setObjectForm:)]
        pub unsafe fn setObjectForm(&self, object_form: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSTermOfAddress instead"]
        #[method_id(@__retain_semantics Other possessiveForm)]
        pub unsafe fn possessiveForm(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSTermOfAddress instead"]
        #[method(setPossessiveForm:)]
        pub unsafe fn setPossessiveForm(&self, possessive_form: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSTermOfAddress instead"]
        #[method_id(@__retain_semantics Other possessiveAdjectiveForm)]
        pub unsafe fn possessiveAdjectiveForm(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSTermOfAddress instead"]
        #[method(setPossessiveAdjectiveForm:)]
        pub unsafe fn setPossessiveAdjectiveForm(
            &self,
            possessive_adjective_form: Option<&NSString>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSTermOfAddress instead"]
        #[method_id(@__retain_semantics Other reflexiveForm)]
        pub unsafe fn reflexiveForm(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSTermOfAddress instead"]
        #[method(setReflexiveForm:)]
        pub unsafe fn setReflexiveForm(&self, reflexive_form: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSMorphologyCustomPronoun")]
    unsafe impl NSMorphologyCustomPronoun {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSMorphologyUserSettings
    #[cfg(feature = "Foundation_NSMorphology")]
    unsafe impl NSMorphology {
        #[method(isUnspecified)]
        pub unsafe fn isUnspecified(&self) -> bool;

        #[method_id(@__retain_semantics Other userMorphology)]
        pub unsafe fn userMorphology() -> Id<NSMorphology>;
    }
);
