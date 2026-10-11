//! The protocol IEs of the message of a PDU, whatever the message: read, and put back.

use rasn::types::Any;
use rasn::{AsnType, Decoder, Encode};

use crate::Error;
use crate::check::object_set;
use crate::s1ap::{ProtocolIEContainer, S1AP_PDU};

/// A message as the ASN.1 has every message but the private one: a SEQUENCE of one
/// container of protocol IEs, which a later release may add to.
#[derive(AsnType, Encode)]
#[rasn(automatic_tags)]
#[non_exhaustive]
struct AnyMessage {
    #[rasn(identifier = "protocolIEs")]
    protocol_ies: ProtocolIEContainer,
}
crate::per::decode_extensible_sequence! { AnyMessage {
    #[rasn(identifier = "protocolIEs")]
    protocol_ies: [ProtocolIEContainer],
} }

impl S1AP_PDU {
    /// The open type that holds the message of this PDU.
    fn value(&self) -> &Any {
        match self {
            Self::initiatingMessage(message) => &message.value,
            Self::successfulOutcome(message) => &message.value,
            Self::unsuccessfulOutcome(message) => &message.value,
        }
    }

    /// The name that ASN.1 gives the message of this PDU, when the specification has
    /// its procedure in that direction. An error for a message that has no container
    /// of protocol IEs.
    fn message_of_protocol_ies(&self) -> Result<Option<&'static str>, Error> {
        match object_set(self.direction(), self.procedure_code()) {
            Some(set) if set.ies.is_none() => Err(Error::no_protocol_ies(set.name)),
            set => Ok(set.map(|set| set.name)),
        }
    }

    /// The protocol IEs of the message of this PDU, whatever the message is: the
    /// container of the bindings, in which each IE is as it was received, with its
    /// identifier, its criticality and the octets of its value. An IE that the message
    /// cannot have, one that the protocol does not have and one that is there twice are
    /// all in it, in the order of the message.
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
    pub fn protocol_ies(&self) -> Result<ProtocolIEContainer, Error> {
        let name = self.message_of_protocol_ies()?;
        let message: AnyMessage = crate::per::decode_complete(self.value().as_bytes())
            .map_err(|error| error.of_message(name))?;
        Ok(message.protocol_ies)
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
    /// // Its IEs, with one more
    /// let mut ies = pdu.protocol_ies()?;
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
    pub fn set_protocol_ies(&mut self, protocol_ies: ProtocolIEContainer) -> Result<(), Error> {
        self.message_of_protocol_ies()?;
        let count = protocol_ies.0.len();
        let value = crate::per::encode_open_type(&AnyMessage { protocol_ies })
            .map_err(|error| error.of_protocol_ies(count))?;
        match self {
            Self::initiatingMessage(message) => message.value = value,
            Self::successfulOutcome(message) => message.value = value,
            Self::unsuccessfulOutcome(message) => message.value = value,
        }
        Ok(())
    }
}
