# Hazard records

- `quakes_m55.csv`: earthquakes of magnitude 5.5 and above, 1900 to 2024, from the USGS ComCat catalogue
  (`earthquake.usgs.gov/fdsnws/event/1/query`, public domain). Year, latitude, longitude, magnitude.
  The catalogue is incomplete before about 1960 for the smaller sizes. It sets the long-term moment
  supply of each tile and is the yardstick for the size-frequency test.
- `eruptions.csv`: significant volcanic eruptions since 8000 BC with a Volcanic Explosivity Index, from the
  NOAA NCEI Natural Hazards "Significant Volcanic Eruption Database" (`ngdc.noaa.gov/hazel`). "Significant"
  means it caused deaths, damage, a tsunami or was VEI 6 or more, so small remote eruptions are missing and
  the record undercounts. Year, latitude, longitude, VEI, name.
