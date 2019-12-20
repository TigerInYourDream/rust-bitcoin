//! mod for bloom filter message
#[derive(PartialEq, Eq, Clone, Debug)]
///the message filterload
pub struct FilterLoadMessage {
    ///The filter itself is simply a bit field of arbitrary byte-aligned size. The maximum size is 36,000 bytes.
    pub  filter: Vec<u8>,
    ///The number of hash functions to use in this filter. The maximum value allowed in this field is 50.
    pub n_hash_functions: u32,
    ///A random value to add to the seed value in the hash function used by the bloom filter.
    pub n_tweak: u32,
    ///A set of flags that control how matched items are added to the filter.
    pub n_flags: bool,
}

impl_consensus_encoding!(FilterLoadMessage, filter, n_hash_functions, n_tweak, n_flags);

#[cfg(test)]
mod test {
    use network::message_bloom_filter::FilterLoadMessage;
    use consensus::{deserialize, serialize};
    use network::message::{RawNetworkMessage, NetworkMessage};

    #[test]
    fn serialize_filterload_test() {
        let data = vec![
            0xf9, 0xbe, 0xb4, 0xd9, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x6c, 0x6f,
            0x61, 0x64, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x8b, 0x7f, 0x50, 0x7b,
            0x02, 0xb5, 0x0f, 0x0b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let filterload = FilterLoadMessage {
            filter: vec![0xb5, 0x0f],
            n_hash_functions: 11,
            n_tweak: 0,
            n_flags: false,
        };

        let raw_filterload = RawNetworkMessage {
            magic: 0xd9b4bef9,
            payload: NetworkMessage::FilterLoad(filterload),
        };

        let raw_data = deserialize::<RawNetworkMessage>(&data);
        assert!(&raw_data.is_ok());
        assert_eq!(data, serialize(&raw_filterload));
    }
}