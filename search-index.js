var N = null;var searchIndex = {};
searchIndex["const_time_enc"]={"doc":"","items":[[4,"ErrorKind","const_time_enc","Errors that can be returned by encoding and decoding operations.",N,N],[13,"InvalidEncodingChar","","An encoded string contained a byte value not valid for the encoding alphabet.",0,N],[13,"BadPadding","","The padding is incorrect for the length of the output. Only applicable to Base64 and Base32.",0,N],[0,"base64","","",N,N],[5,"base64_encoded_max_size","const_time_enc::base64","Give the upper bound for the size of the buffer needed to contain the result of encoding a bytestring `sz` bytes long into base64.",N,[[["usize"]],["usize"]]],[5,"base64_decoded_max_size","","Give the upper bound for the size of the buffer needed to contain the result of decoding a base64 string `sz` bytes long.",N,[[["usize"]],["usize"]]],[5,"base64_encode","","Encode a bytestring into base64 in constant-time.",N,N],[5,"base64_decode","","Decode a base64 encoded bytestring in constant time.",N,N],[11,"fmt","const_time_enc","",0,[[["self"],["formatter"]],["result"]]],[11,"clone","","",0,[[["self"]],["errorkind"]]],[6,"Result","","",N,N],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[["self"]],["u"]]],[11,"to_owned","","",0,[[["self"]],["t"]]],[11,"clone_into","","",0,N],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"get_type_id","","",0,[[["self"]],["typeid"]]],[11,"try_into","","",0,[[["self"]],["result"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]]],"paths":[[4,"ErrorKind"]]};
initSearch(searchIndex);