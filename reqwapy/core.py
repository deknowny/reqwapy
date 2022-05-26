# Maturin's failure,
# it can't create a rust nuild with another name.
# All this code was written on pure rust
from reqwapy.reqwapy import client, response

Client = client.Client
TreatResponseAs = client.TreatResponseAs

JSONResponse = response.JSONResponse
TextResponse = response.TextResponse
RawResponse = response.RawResponse
