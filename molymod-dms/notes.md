`.schema`:
```sql
CREATE TABLE msys_ct ('msys_name' text, id integer primary key);
CREATE TABLE global_cell (
  id integer primary key,
  x float, y float, z float);
CREATE TABLE particle (
  id integer primary key,
  anum integer,
  name text not null,
  x float,
  y float,
  z float,
  vx float,
  vy float,
  vz float,
  resname text not null,
  resid integer,
  chain text not null,
  segid text not null,
  mass float,
  charge float,
  formal_charge integer,
  resonant_charge float,
  insertion text not null,
  msys_ct integer not null,
  'grp_temperature' integer,
  'grp_energy' integer,
  'grp_ligand' integer,
  'grp_bias' integer);
CREATE TABLE bond (
  p0 integer,
  p1 integer,
  'order' integer,
  resonant_order float
);
CREATE TABLE msys_hash(system text);
CREATE TABLE provenance (
  id integer primary key,
  version text,
  timestamp text,
  user text,
  workdir text,
  cmdline text,
  executable text);
CREATE TABLE dms_version (
  major integer not null,
  minor integer not null);
```