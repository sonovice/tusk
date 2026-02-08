# Disabled integration tests

`schema_validation.rs` is here (not in `tests/`) so it is not compiled. It targets the previous ODD-generated model and uses types that the current RNG-generated internal model does not have (e.g. `StaffChild`, `BeamChild`, `AttHairpinLogForm`). Move it back to `tests/` and fix imports/type usage to align with the RNG model when re-enabling.
