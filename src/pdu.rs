//! The protocol IEs of the message of a PDU, whatever the message: read, put back, and
//! compared with the object set that the ASN.1 gives the message. What does it is
//! `oxirush_aper::protocol`, which the protocols share: here are the methods of the PDU.

use oxirush_aper::protocol::{self, Ie, Pdu, Source};
use oxirush_aper::{Error, Octets};

use crate::Deviation;
use crate::S1ap;
use crate::s1ap::{Criticality, CriticalityDiagnostics, ProtocolIEContainer, S1APPDU};

impl<'a> S1APPDU<'a> {
    /// The octets of the open type that holds the message of this PDU.
    #[inline]
    pub(crate) fn value(&self) -> &Octets<'a> {
        <Self as Pdu<'a>>::value(self)
    }

    /// The PDU of a direction, as `direction()` says it, with the code of its procedure,
    /// its criticality and the octets of the open type that holds its message; `None`
    /// for what is no direction.
    #[cfg(feature = "inspect")]
    pub(crate) fn of(
        direction: &str,
        procedure_code: u8,
        criticality: Criticality,
        value: Octets<'a>,
    ) -> Option<Self> {
        <Self as Pdu<'a>>::__of(direction, procedure_code, criticality, value)
    }

    /// The protocol IEs of the message of this PDU, whatever the message is: the
    /// container of the bindings, in which each IE is as it was received, with its
    /// identifier, its criticality and the octets of its value, which are borrowed
    /// from the PDU. An IE that the message cannot have, one that the protocol does not
    /// have and one that is there twice are all in it, in the order of the message.
    ///
    /// Every message of TS 36.413 but the private one is a container of protocol IEs,
    /// so this needs no type: `decode_value` takes the type of the message that the PDU
    /// holds, and refuses another one. The message of a procedure that the
    /// specification does not have in that direction is read as a container of
    /// protocol IEs too.
    ///
    /// ```
    /// use oxirush_s1ap::s1ap::*;
    ///
    /// // The eNB UE S1AP ID of a PDU, whatever its message, when it has one
    /// fn enb_ue_id(pdu: &S1AP_PDU) -> Result<Option<u32>, oxirush_s1ap::Error> {
    ///     let ies = pdu.protocol_ies()?;
    ///     let Some(ie) = ies.0.iter().find(|ie| ie.id == ID_E_NB_UE_S1_AP_ID) else {
    ///         return Ok(None);
    ///     };
    ///     let id: ENBUES1APID = decode_open_type(&ie.value)?;
    ///     Ok(Some(id.0))
    /// }
    ///
    /// // A UE CONTEXT RELEASE REQUEST, then an S1 SETUP FAILURE
    /// let request = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
    /// assert_eq!(enb_ue_id(&S1AP_PDU::decode(&request)?)?, Some(7));
    /// let failure = hex::decode("401100080000010002400145").unwrap();
    /// assert_eq!(enb_ue_id(&S1AP_PDU::decode(&failure)?)?, None);
    /// # Ok::<(), oxirush_s1ap::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// An error of the kind `WrongMessage` for the private message, which has private
    /// IEs and no container of protocol IEs, and one of the kind `Decode` when the
    /// octets of the message are not a container of protocol IEs.
    pub fn protocol_ies(&self) -> Result<ProtocolIEContainer<'_>, Error> {
        protocol::protocol_ies(self).map(ProtocolIEContainer)
    }

    /// Make `protocol_ies` the protocol IEs of the message of this PDU: the PDU keeps
    /// its direction, its procedure code and its criticality, and its message is these
    /// IEs, in their order and as they are. With [`protocol_ies`](Self::protocol_ies),
    /// this amends a message of any type:
    ///
    /// ```
    /// use oxirush_s1ap::{build_s1ap_ie, s1ap::*};
    ///
    /// // An S1 SETUP FAILURE
    /// let failure = hex::decode("401100080000010002400145").unwrap();
    /// let mut pdu = S1AP_PDU::decode(&failure)?;
    ///
    /// // Its IEs, with one more: they are owned, as the PDU is changed
    /// let mut ies = pdu.protocol_ies()?.into_owned();
    /// let wait = build_s1ap_ie!(S1SetupFailure, IGNORE TimeToWait(TimeToWait::v5s))?;
    /// ies.0.push(wait);
    /// pdu.set_protocol_ies(ies)?;
    ///
    /// assert_eq!(pdu.kind(), S1apPduKind::Unsuccessful_S1Setup);
    /// let failure: S1SetupFailure = pdu.decode_value()?;
    /// assert_eq!(failure.protocol_ies.0.len(), 2);
    /// # Ok::<(), oxirush_s1ap::Error>(())
    /// ```
    ///
    /// Nothing is checked: the IEs are those of any message, in any order.
    ///
    /// # Errors
    ///
    /// An error of the kind `WrongMessage` for the private message, which has no
    /// container of protocol IEs, and one of the kind `Encode` for more IEs than a
    /// container has, which is 65535. The PDU is as it was after an error.
    pub fn set_protocol_ies(&mut self, protocol_ies: ProtocolIEContainer<'_>) -> Result<(), Error> {
        protocol::set_protocol_ies(self, &protocol_ies.0)
    }

    /// Every occurrence of the IE `T` in the message of this PDU, in its order, each decoded
    /// as the type of `T`. The octets of the message are read where they are, without
    /// `decode_value` and without allocating, and an IE of another identifier is passed
    /// over and not decoded: a value that does not decode is an error of the kind
    /// `UndecodableIe`, and octets that are no container of protocol IEs an error of the
    /// kind `Decode`, which ends the iterator.
    ///
    /// An IE that the message has twice is there twice: `check()` reports the second as
    /// [`Deviation::RepeatedIe`](oxirush_aper::check::Deviation::RepeatedIe), and
    /// [`ie`](Self::ie) takes the first.
    ///
    /// ```
    /// use oxirush_s1ap::{build_s1ap, ie, s1ap::*};
    ///
    /// // A UE CONTEXT RELEASE REQUEST with its eNB-UE-S1AP-ID twice
    /// let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest, UEContextReleaseRequest,
    ///     MME_UE_S1AP_ID(42u32),
    ///     eNB_UE_S1AP_ID(7u32),
    ///     eNB_UE_S1AP_ID(8u32),
    ///     IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    /// )?;
    /// let ids: Vec<_> = pdu.ies::<ie::eNB_UE_S1AP_ID>().collect::<Result<_, _>>()?;
    /// assert_eq!(ids.len(), 2);
    /// # Ok::<(), oxirush_s1ap::Error>(())
    /// ```
    #[inline]
    pub fn ies<T: Ie<Protocol = S1ap>>(&self) -> protocol::Ies<'_, T> {
        Source::__of_pdu(self).ies::<T>()
    }

    /// The first occurrence of the IE `T` in the message of this PDU, decoded as the type
    /// of `T`; `None` when the message does not have it. The octets of the message are read
    /// up to that IE, and nothing else is decoded: see [`ies`](Self::ies).
    ///
    /// ```
    /// use oxirush_s1ap::{ie, s1ap::*};
    ///
    /// // The Cause of a UE CONTEXT RELEASE REQUEST, read in the buffer that it came in
    /// let octets = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
    /// let pdu = S1AP_PDU::decode_borrowed(&octets)?;
    /// let cause = pdu.ie::<ie::Cause>().unwrap()?;
    /// assert_eq!(cause, Cause::radioNetwork(CauseRadioNetwork::user_inactivity));
    ///
    /// // An IE that the message does not have
    /// assert!(pdu.ie::<ie::TimeToWait>().is_none());
    /// # Ok::<(), oxirush_s1ap::Error>(())
    /// ```
    #[inline]
    pub fn ie<T: Ie<Protocol = S1ap>>(&self) -> Option<Result<T::Value<'_>, Error>> {
        self.ies::<T>().next()
    }

    /// The items of the list `L` in the message of this PDU, a list of single containers,
    /// each decoded as the IE of its single container, from the first occurrence of `L`:
    /// the list is walked where it is, and not built. Nothing when the message has no such
    /// list, which [`ie`](Self::ie) tells.
    #[inline]
    pub fn items<L: protocol::List<Protocol = S1ap>>(&self) -> protocol::Items<'_, L> {
        Source::__of_pdu(self).items::<L>()
    }

    /// The PDU that holds `message`: of the direction and the procedure code of the
    /// message, and of the criticality `criticality`, which is that of its procedure for
    /// a PDU as TS 36.413 clause 9.4.1 has a sender construct it. The message is encoded
    /// as it is, whatever its IEs are, as `build_s1ap!` builds one from its IEs.
    ///
    /// ```
    /// use oxirush_s1ap::{build_s1ap, s1ap::*};
    ///
    /// // A message that is at hand, as one that a test changed after it was decoded
    /// let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
    ///     IGNORE, UEContextReleaseRequest,
    ///     REJECT MME_UE_S1AP_ID(1u32),
    ///     REJECT eNB_UE_S1AP_ID(7u32),
    ///     IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    /// )?;
    /// let mut message: UEContextReleaseRequest = pdu.decode_value()?;
    /// message.protocol_ies.0.swap(0, 1);
    ///
    /// let again = S1AP_PDU::of_message(&message, Criticality::ignore)?;
    /// assert_eq!(again.procedure_name(), "UEContextReleaseRequest");
    /// assert_eq!(again.decode_value::<UEContextReleaseRequest>()?, message);
    /// # Ok::<(), oxirush_s1ap::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// A message that does not encode: a value outside a constraint of its type.
    pub fn of_message<'m, M: crate::Message<'m, Protocol = S1ap>>(
        message: &M,
        criticality: Criticality,
    ) -> Result<S1APPDU<'static>, Error> {
        protocol::of_message(message, criticality)
    }

    /// The criticality of this PDU, which the specification assigns to its procedure
    /// (`Message::CRITICALITY` of its message) in a PDU that is constructed as it has a
    /// sender construct one.
    pub fn criticality(&self) -> Criticality {
        <Self as Pdu<'a>>::criticality(self)
    }

    /// How this PDU deviates from what the ASN.1 assigns its message: the criticality of
    /// its procedure, and the object set of its IEs. The list is empty for a PDU that is
    /// constructed as TS 36.413 clause 9.3.0 has a sender construct it.
    ///
    /// The deviations, and what `check` does not look at, are those of
    /// [`oxirush_aper::protocol::check`], where each variant of [`Deviation`] is told. A
    /// deviation says what the receiver of the message answers: the cause of
    /// [`Deviation::cause`], and for the Criticality Diagnostics the item of
    /// [`Deviation::diagnostics_item`], which
    /// [`criticality_diagnostics`](Self::criticality_diagnostics) gathers into the IE of
    /// the answer. Clause 10.3 of TS 36.413 has what a receiver does.
    ///
    /// `check` decodes the value of each IE of the object set. A receiver that reads the
    /// IEs that it needs, which their decoding checks, calls
    /// [`check_structure`](Self::check_structure), which does not.
    ///
    /// ```
    /// use oxirush_s1ap::{Deviation, build_s1ap, s1ap::*};
    ///
    /// // A UE CONTEXT RELEASE REQUEST without its Cause, and with its two identifiers
    /// // in the other order
    /// let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
    ///     IGNORE, UEContextReleaseRequest,
    ///     REJECT eNB_UE_S1AP_ID(7u32),
    ///     REJECT MME_UE_S1AP_ID(42u32),
    /// )?;
    ///
    /// // It is built and encoded as any other, and `check` says what it deviates in
    /// let received = S1AP_PDU::decode(&pdu.encode()?)?;
    /// let deviations = received.check()?;
    /// assert_eq!(deviations.len(), 2);
    /// assert!(matches!(deviations[0], Deviation::OutOfOrder { id: 0, after: 8, .. }));
    /// assert!(matches!(deviations[1], Deviation::MissingIe { id: 2, name: "Cause", .. }));
    /// assert_eq!(
    ///     deviations[1].to_string(),
    ///     "Cause (IE 2) is mandatory, and the message does not have it"
    /// );
    /// # Ok::<(), oxirush_s1ap::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// An error of the kind `Decode` when the octets of the message are not a container
    /// of protocol IEs.
    pub fn check(&self) -> Result<Vec<Deviation>, Error> {
        protocol::check(self)
    }

    /// How this PDU deviates from what the ASN.1 assigns its message, in the container of
    /// its IEs alone: the criticality of its procedure, and the object set of its IEs, the
    /// presence, the order, the repetition and the criticality of each, which is what
    /// clause 10.3 of TS 36.413 has a receiver answer. The values of the IEs are not
    /// decoded, and what [`check`](Self::check) reports of them is not here:
    /// `UndecodableIe`, `OutsideRoot` and `NotCanonical`.
    ///
    /// A receiver that checks the structure and reads the IEs that it needs, with
    /// `extract_s1ap_ies!` or [`ie`](Self::ie), pays for those IEs alone: the decoding of
    /// each checks its value, and an IE that does not decode is the error of the kind
    /// `UndecodableIe` there, which [`Deviation::of`] makes a deviation of. `check`
    /// decodes every IE of the object set, for a tool that looks at every value.
    ///
    /// ```
    /// use oxirush_s1ap::{build_s1ap, s1ap::*};
    ///
    /// // A UE CONTEXT RELEASE REQUEST without its Cause
    /// let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest, UEContextReleaseRequest,
    ///     MME_UE_S1AP_ID(42u32),
    ///     eNB_UE_S1AP_ID(7u32),
    /// )?;
    /// let deviations = pdu.check_structure()?;
    /// assert_eq!(deviations[0].to_string(), "Cause (IE 2) is mandatory, and the message does not have it");
    /// # Ok::<(), oxirush_s1ap::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// An error of the kind `Decode` when the octets of the message are not a container
    /// of protocol IEs.
    pub fn check_structure(&self) -> Result<Vec<Deviation>, Error> {
        protocol::check_structure(self)
    }

    /// The Criticality Diagnostics IE of the ERROR INDICATION with which the receiver of
    /// this PDU answers it for `deviations`, which [`check`](Self::check) gave: the
    /// procedure code, the triggering message and the procedure criticality of this PDU,
    /// which TS 36.413 clause 9.2.1.21 has in an ERROR INDICATION alone, as clause 10.3.4.1
    /// has them for a procedure that the receiver does not comprehend, and the item of
    /// each deviation that has one, as clauses 10.3.4.2 and 10.3.5 have them for an IE,
    /// 256 at most. No list of IEs where no deviation has an item. The response of the
    /// procedure of this PDU has
    /// [`criticality_diagnostics_for_response`](Self::criticality_diagnostics_for_response).
    ///
    /// ```
    /// use oxirush_s1ap::{build_s1ap, s1ap::*};
    ///
    /// // A UE CONTEXT RELEASE REQUEST without its eNB UE S1AP ID, which is mandatory and
    /// // of the criticality reject: the procedure has no response
    /// let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest, IGNORE,
    ///     UEContextReleaseRequest,
    ///     REJECT MME_UE_S1AP_ID(42u32),
    ///     IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    /// )?;
    /// let deviations = pdu.check()?;
    /// let diagnostics = pdu.criticality_diagnostics(&deviations);
    /// assert_eq!(diagnostics.procedure_code, Some(ProcedureCode(18)));
    /// assert_eq!(diagnostics.triggering_message, Some(TriggeringMessage::initiating_message));
    /// assert_eq!(diagnostics.procedure_criticality, Some(Criticality::ignore));
    /// let items = diagnostics.ies_criticality_diagnostics.unwrap();
    /// assert_eq!(items.0[0].ie_id, ProtocolIEID(8));
    /// assert_eq!(items.0[0].type_of_error, TypeOfError::missing);
    /// # Ok::<(), oxirush_s1ap::Error>(())
    /// ```
    pub fn criticality_diagnostics(&self, deviations: &[Deviation]) -> CriticalityDiagnostics {
        oxirush_aper::check::criticality_diagnostics(
            self.procedure_code(),
            self.direction(),
            self.criticality(),
            deviations,
        )
    }

    /// The Criticality Diagnostics IE of the response of the procedure of this PDU, with
    /// which its receiver answers it for `deviations`: the procedure criticality of this
    /// PDU and the item of each deviation that has one, without the procedure code and the
    /// triggering message, which TS 36.413 clause 9.2.1.21 keeps for an ERROR INDICATION.
    ///
    /// ```
    /// use oxirush_s1ap::{build_s1ap, s1ap::*};
    ///
    /// // An S1 SETUP REQUEST without its Global eNB ID, which is mandatory and of
    /// // the criticality reject: the S1 SETUP FAILURE says so
    /// let pdu = build_s1ap!(InitiatingMessage, S1Setup, REJECT, S1SetupRequest,
    ///     IGNORE DefaultPagingDRX(PagingDRX::v128),
    /// )?;
    /// let deviations = pdu.check()?;
    /// let diagnostics = pdu.criticality_diagnostics_for_response(&deviations);
    /// assert_eq!(diagnostics.procedure_code, None);
    /// assert_eq!(diagnostics.triggering_message, None);
    /// assert_eq!(diagnostics.procedure_criticality, Some(Criticality::reject));
    /// let items = diagnostics.ies_criticality_diagnostics.unwrap();
    /// assert_eq!(items.0[0].ie_id, ProtocolIEID(59));
    /// assert_eq!(items.0[0].type_of_error, TypeOfError::missing);
    /// # Ok::<(), oxirush_s1ap::Error>(())
    /// ```
    pub fn criticality_diagnostics_for_response(
        &self,
        deviations: &[Deviation],
    ) -> CriticalityDiagnostics {
        oxirush_aper::check::criticality_diagnostics_for_response(self.criticality(), deviations)
    }
}
