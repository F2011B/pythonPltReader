import unittest
import tecplotPltReader


class TestStringMethods(unittest.TestCase):

    def test_construct_dword(self):
        bytes_to_parse = b"\x01\x00\x00\x00\x00\x00\x00\x01"
        qword_res = tecplotPltReader.construct_qword(bytes_to_parse)
        target=72057594037927937
        self.assertEqual(qword_res['Correct'], True)
        self.assertEqual(qword_res['qword'], target)
        bytes_to_parse = b"\x01\x00\x00"
        qword_res = tecplotPltReader.construct_qword(bytes_to_parse)
        self.assertEqual(qword_res['Correct'], False)

    def test_construct_qword_for_TecStr(self):
        bytes_to_parse = b"\x2e\x00\x00\x00\x2e\x00\x00\x00"
        qword_res = tecplotPltReader.construct_qword(bytes_to_parse)
        self.assertEqual(qword_res['Correct'],True)
        self.assertEqual(qword_res['tec_str'], '..')

    def test_read_magic_number(self):
        #[hex(0x12345678 >> i & 0xff) for i in (24, 16, 8, 0)]
        mag_res = tecplotPltReader.read_magic_number(b"\x23\x21\x54\x44\x56\x31\x31\x32")
        self.assertEqual(mag_res['Correct'], True)
        self.assertEqual(mag_res['uni_chars'], '#!TDV112')

    def test_read_header(self):
        #[hex(0x12345678 >> i & 0xff) for i in (24, 16, 8, 0)]
        mag_res = tecplotPltReader.read_header(b"\x23\x21\x54\x44\x56\x31\x31\x32"
                                               b"\x01\x00\x00\x00\x00\x00\x00\x00"        
                                               b"\x2e\x00\x00\x00\x2e\x00\x00\x00"
                                               b"\x2e\x00\x00\x00\x00\x00\x00\x00"                                               
                                               b"\x2f\x00\x00\x00\x50\x00\x00\x00"
                                               b"\x69\x00\x00\x00\x63\x00\x00\x00"
                                               b"\x74\x00\x00\x00\x75\x00\x00\x00"
                                               b"\x72\x00\x00\x00\x65\x00\x00\x00"
                                               b"\x00\x00\x00\x00\x78\x00\x00\x00")

        self.assertEqual(mag_res['Correct'], True)
        self.assertEqual(mag_res['magic_num']['uni_chars'], '#!TDV112')
        self.assertEqual(mag_res['ByteOrder'], 1)
        self.assertEqual(mag_res['FileType'], 'FULL')
        self.assertEqual(mag_res['NumVars'], 47)
        self.assertEqual(mag_res['Title'], '...')


if __name__ == '__main__':
    unittest.main()
