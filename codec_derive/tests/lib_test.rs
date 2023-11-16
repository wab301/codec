#[cfg(test)]
mod tests {
    use codec_derive::{Deserialize, Serialize};
    use bytes::{self, BytesMut, BufMut, Buf};

    #[derive(Deserialize,Serialize,Debug,Default)]
    struct TestStruct {
        pub num1: f32,
        pub num2: f64,
        pub num3: i16,
        pub num4: i32,
        pub num5: i64,
        pub num6: u16,
        pub num7: u32,
        pub num8: u64,
        pub num9: i8,
        pub num10: u8,
        pub nums1: Vec<f32>,
        pub nums2: Vec<f64>,
        pub nums3: Vec<i16>,
        pub nums4: Vec<i32>,
        pub nums5: Vec<i64>,
        pub nums6: Vec<u16>,
        pub nums7: Vec<u32>,
        pub nums8: Vec<u64>,
        pub nums9: Vec<i8>,
        pub nums10: Vec<u8>,
        pub str1: String,
        pub strs1: Vec<String>,
        pub object: TestSubStruct,
        pub objects: Vec<TestSubStruct>,
    }

    impl PartialEq for TestStruct {
        fn eq(&self, other: &Self) -> bool {
            self.num1 == other.num1 && self.num2 == other.num2 && self.num3 == other.num3 &&
            self.num4 == other.num4 && self.num5 == other.num5 && self.num6 == other.num6 &&
            self.num7 == other.num7 && self.num8 == other.num8 && self.num9 == other.num9 &&
            self.num10 == other.num10 && self.nums1 == other.nums1 && self.nums2 == other.nums2 &&
            self.nums3 == other.nums3 && self.nums4 == other.nums4 && self.nums5 == other.nums5 && 
            self.nums6 == other.nums6 && self.nums7 == other.nums7 && self.nums8 == other.nums8 && 
            self.nums9 == other.nums9 && self.nums10 == other.nums10 && self.str1 == other.str1 &&
            self.strs1 == other.strs1 && self.object == other.object && self.objects == other.objects
        }
    }

    #[derive(Deserialize,Serialize,Debug,Default)]
    struct TestSubStruct {
        pub num1: i8,
    }

    impl PartialEq for TestSubStruct {
        fn eq(&self, other: &Self) -> bool {
            self.num1 == other.num1
        }
    }

    #[test]
    fn test_deserialize() {
        let mut buf: BytesMut = BytesMut::new();
        buf.put_f32_le(1.23);
        buf.put_f64_le(333.12345678);
        buf.put_i16_le(12345);
        buf.put_i32_le(1234567);
        buf.put_i64_le(1234567890);
        buf.put_u16_le(65535);
        buf.put_u32_le(1234567);
        buf.put_u64_le(1234567890);
        buf.put_i8(127);
        buf.put_u8(255);

        buf.put_i32_le(1);
        buf.put_f32_le(1.23);
        buf.put_i32_le(1);
        buf.put_f64_le(333.12345678);
        buf.put_i32_le(1);
        buf.put_i16_le(12345);
        buf.put_i32_le(1);
        buf.put_i32_le(1234567);
        buf.put_i32_le(1);
        buf.put_i64_le(1234567890);
        buf.put_i32_le(1);
        buf.put_u16_le(65535);
        buf.put_i32_le(1);
        buf.put_u32_le(1234567);
        buf.put_i32_le(1);
        buf.put_u64_le(1234567890);
        buf.put_i32_le(1);
        buf.put_i8(127);
        buf.put_i32_le(1);
        buf.put_u8(255);

        buf.put_u16_le(4);
        buf.put_bytes(b'a', 4);

        buf.put_i32_le(1);
        buf.put_u16_le(5);
        buf.put_bytes(b'b', 5);

        buf.put_i8(1);

        buf.put_i32_le(2);
        buf.put_i8(2);
        buf.put_i8(3);

        let test_struct = TestStruct::deserialize(&mut buf);
        assert_eq!(test_struct.num1, 1.23);
        assert_eq!(test_struct.num2, 333.12345678);
        assert_eq!(test_struct.num3, 12345);
        assert_eq!(test_struct.num4, 1234567);
        assert_eq!(test_struct.num5, 1234567890);
        assert_eq!(test_struct.num6, 65535);
        assert_eq!(test_struct.num7, 1234567);
        assert_eq!(test_struct.num8, 1234567890);
        assert_eq!(test_struct.num9, 127);
        assert_eq!(test_struct.num10, 255);
        assert_eq!(test_struct.nums1, vec![1.23]);
        assert_eq!(test_struct.nums2, vec![333.12345678]);
        assert_eq!(test_struct.nums3, vec![12345]);
        assert_eq!(test_struct.nums4, vec![1234567]);
        assert_eq!(test_struct.nums5, vec![1234567890]);
        assert_eq!(test_struct.nums6, vec![65535]);
        assert_eq!(test_struct.nums7, vec![1234567]);
        assert_eq!(test_struct.nums8, vec![1234567890]);
        assert_eq!(test_struct.nums9, vec![127]);
        assert_eq!(test_struct.nums10, vec![255]);

        assert_eq!(test_struct.str1, "aaaa");
        assert_eq!(test_struct.strs1, vec!["bbbbb"]);
        assert_eq!(test_struct.object, TestSubStruct{num1: 1});
        assert_eq!(test_struct.objects, vec![TestSubStruct{num1: 2}, TestSubStruct{num1: 3}]);
    }

    #[test]
    fn test_serialize() {
        let mut object = TestStruct {
            num1: 1.23,
            num2: 333.12345678,
            num3: 12345,
            num4: 1234567,
            num5: 1234567890,
            num6: 65535,
            num7: 1234567,
            num8: 1234567890,
            num9: 127,
            num10: 255,
            nums1: vec![1.23],
            nums2: vec![333.12345678],
            nums3: vec![12345],
            nums4: vec![1234567],
            nums5: vec![1234567890],
            nums6: vec![65535],
            nums7: vec![1234567],
            nums8: vec![1234567890],
            nums9: vec![127],
            nums10: vec![255],
            str1: String::from("aaaa"),
            strs1: vec![String::from("bbbbb")],
            object: TestSubStruct{num1:1},
            objects: vec![TestSubStruct{num1:2},TestSubStruct{num1:3}],
        };
        
        let mut buf = BytesMut::new();
        object.serialize(&mut buf);
        
        let new_object = TestStruct::deserialize(&mut buf);
        assert_eq!(object, new_object);
    }
}