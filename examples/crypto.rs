use validators::crypto;

fn main() {
    println!("1GiWxH6PzSSmbdcK72XfGpqhjSb6nae6h9 => {:?}", crypto::which_cryptocurrency("1GiWxH6PzSSmbdcK72XfGpqhjSb6nae6h9"));
    println!("qppjlghjlwg6tgxv7ffhvs43rlul0kpp4c0shk4dr6 => {:?}", crypto::which_cryptocurrency("qppjlghjlwg6tgxv7ffhvs43rlul0kpp4c0shk4dr6"));
    println!("0xaae47eae4ddd4877e0ae0bc780cfaee3cc3b52cb => {:?}", crypto::which_cryptocurrency("0xaae47eae4ddd4877e0ae0bc780cfaee3cc3b52cb"));
    println!("LQ4i7FLNhfCC9GXw682mS1NzvVKbtJAFZq => {:?}", crypto::which_cryptocurrency("LQ4i7FLNhfCC9GXw682mS1NzvVKbtJAFZq"));
    println!("D6K2nqqQKycTucCSFSHhpiig4yQ6NPQRf9 => {:?}", crypto::which_cryptocurrency("D6K2nqqQKycTucCSFSHhpiig4yQ6NPQRf9"));
    println!("XqLYPDTADW6EYuQmTcEAx81o8EHTKwqTK8 => {:?}", crypto::which_cryptocurrency("XqLYPDTADW6EYuQmTcEAx81o8EHTKwqTK8"));
    println!("41gYNjXMeXaTmZFVv645A1HRVoA637cXFGbDdLV8Gn5hLvfxfRLKigUTvm2HVZhBzDVPeGpDy71qxASTpRFgepDwLexA8Ti => {:?}", crypto::which_cryptocurrency("41gYNjXMeXaTmZFVv645A1HRVoA637cXFGbDdLV8Gn5hLvfxfRLKigUTvm2HVZhBzDVPeGpDy71qxASTpRFgepDwLexA8Ti"));
    println!("AeHauBkGkHPTxh4PEUhNr7WRgivmcdCRnR => {:?}", crypto::which_cryptocurrency("AeHauBkGkHPTxh4PEUhNr7WRgivmcdCRnR"));
    println!("rUocf1ixKzTuEe34kmVhRvGqNCofY1NJzV => {:?}", crypto::which_cryptocurrency("rUocf1ixKzTuEe34kmVhRvGqNCofY1NJzV"));
}