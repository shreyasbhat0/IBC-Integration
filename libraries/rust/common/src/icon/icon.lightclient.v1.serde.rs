// @generated
impl serde::Serialize for BlockUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.header.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("icon.lightclient.v1.BlockUpdate", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlockUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "header" => Ok(GeneratedField::Header),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlockUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct icon.lightclient.v1.BlockUpdate")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BlockUpdate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                    }
                }
                Ok(BlockUpdate {
                    header: header__,
                })
            }
        }
        deserializer.deserialize_struct("icon.lightclient.v1.BlockUpdate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClientState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.trusting_period != 0 {
            len += 1;
        }
        if self.frozen_height != 0 {
            len += 1;
        }
        if self.max_clock_drift != 0 {
            len += 1;
        }
        if self.latest_height != 0 {
            len += 1;
        }
        if !self.network_section_hash.is_empty() {
            len += 1;
        }
        if !self.validators.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("icon.lightclient.v1.ClientState", len)?;
        if self.trusting_period != 0 {
            struct_ser.serialize_field("trusting_period", ToString::to_string(&self.trusting_period).as_str())?;
        }
        if self.frozen_height != 0 {
            struct_ser.serialize_field("frozen_height", ToString::to_string(&self.frozen_height).as_str())?;
        }
        if self.max_clock_drift != 0 {
            struct_ser.serialize_field("max_clock_drift", ToString::to_string(&self.max_clock_drift).as_str())?;
        }
        if self.latest_height != 0 {
            struct_ser.serialize_field("latest_height", ToString::to_string(&self.latest_height).as_str())?;
        }
        if !self.network_section_hash.is_empty() {
            struct_ser.serialize_field("network_section_hash", pbjson::private::base64::encode(&self.network_section_hash).as_str())?;
        }
        if !self.validators.is_empty() {
            struct_ser.serialize_field("validators", &self.validators.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClientState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "trusting_period",
            "trustingPeriod",
            "frozen_height",
            "frozenHeight",
            "max_clock_drift",
            "maxClockDrift",
            "latest_height",
            "latestHeight",
            "network_section_hash",
            "networkSectionHash",
            "validators",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TrustingPeriod,
            FrozenHeight,
            MaxClockDrift,
            LatestHeight,
            NetworkSectionHash,
            Validators,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "trustingPeriod" | "trusting_period" => Ok(GeneratedField::TrustingPeriod),
                            "frozenHeight" | "frozen_height" => Ok(GeneratedField::FrozenHeight),
                            "maxClockDrift" | "max_clock_drift" => Ok(GeneratedField::MaxClockDrift),
                            "latestHeight" | "latest_height" => Ok(GeneratedField::LatestHeight),
                            "networkSectionHash" | "network_section_hash" => Ok(GeneratedField::NetworkSectionHash),
                            "validators" => Ok(GeneratedField::Validators),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClientState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct icon.lightclient.v1.ClientState")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClientState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut trusting_period__ = None;
                let mut frozen_height__ = None;
                let mut max_clock_drift__ = None;
                let mut latest_height__ = None;
                let mut network_section_hash__ = None;
                let mut validators__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TrustingPeriod => {
                            if trusting_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trustingPeriod"));
                            }
                            trusting_period__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FrozenHeight => {
                            if frozen_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("frozenHeight"));
                            }
                            frozen_height__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxClockDrift => {
                            if max_clock_drift__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxClockDrift"));
                            }
                            max_clock_drift__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LatestHeight => {
                            if latest_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("latestHeight"));
                            }
                            latest_height__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NetworkSectionHash => {
                            if network_section_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("networkSectionHash"));
                            }
                            network_section_hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Validators => {
                            if validators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validators"));
                            }
                            validators__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(ClientState {
                    trusting_period: trusting_period__.unwrap_or_default(),
                    frozen_height: frozen_height__.unwrap_or_default(),
                    max_clock_drift: max_clock_drift__.unwrap_or_default(),
                    latest_height: latest_height__.unwrap_or_default(),
                    network_section_hash: network_section_hash__.unwrap_or_default(),
                    validators: validators__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("icon.lightclient.v1.ClientState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConsensusState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message_root.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("icon.lightclient.v1.ConsensusState", len)?;
        if !self.message_root.is_empty() {
            struct_ser.serialize_field("message_root", pbjson::private::base64::encode(&self.message_root).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConsensusState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_root",
            "messageRoot",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MessageRoot,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "messageRoot" | "message_root" => Ok(GeneratedField::MessageRoot),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConsensusState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct icon.lightclient.v1.ConsensusState")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ConsensusState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_root__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MessageRoot => {
                            if message_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageRoot"));
                            }
                            message_root__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ConsensusState {
                    message_root: message_root__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("icon.lightclient.v1.ConsensusState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Misbehaviour {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.client_id.is_empty() {
            len += 1;
        }
        if self.header_1.is_some() {
            len += 1;
        }
        if self.header_2.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("icon.lightclient.v1.Misbehaviour", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("client_id", &self.client_id)?;
        }
        if let Some(v) = self.header_1.as_ref() {
            struct_ser.serialize_field("header_1", v)?;
        }
        if let Some(v) = self.header_2.as_ref() {
            struct_ser.serialize_field("header_2", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Misbehaviour {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
            "header_1",
            "header1",
            "header_2",
            "header2",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
            Header1,
            Header2,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "header1" | "header_1" => Ok(GeneratedField::Header1),
                            "header2" | "header_2" => Ok(GeneratedField::Header2),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Misbehaviour;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct icon.lightclient.v1.Misbehaviour")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Misbehaviour, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut header_1__ = None;
                let mut header_2__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Header1 => {
                            if header_1__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header1"));
                            }
                            header_1__ = map.next_value()?;
                        }
                        GeneratedField::Header2 => {
                            if header_2__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header2"));
                            }
                            header_2__ = map.next_value()?;
                        }
                    }
                }
                Ok(Misbehaviour {
                    client_id: client_id__.unwrap_or_default(),
                    header_1: header_1__,
                    header_2: header_2__,
                })
            }
        }
        deserializer.deserialize_struct("icon.lightclient.v1.Misbehaviour", FIELDS, GeneratedVisitor)
    }
}
