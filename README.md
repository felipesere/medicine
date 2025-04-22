# `medicine` command line

A small app to track medicine usage.
You can add medicines and dosage and adjust the time of the day.

```sh
medicine add ibuprufen
```
adds a single dose of ibuprufen.

```sh
medicine add ibuprufen 2 --at 10:45
```
adds 2x ibuprufen with the time set to 10:45 of today.


You can then see the entire table of medicine and their running totals:
```sh
❯ medicine list all
╭─────────────┬────────┬─────────────────────┬────────────────╮
│ Name        ┆ Dosage ┆ Time Taken          ┆ Since          │
╞═════════════╪════════╪═════════════════════╪════════════════╡
│ ibuprofen   ┆ 1x     ┆ 2025-04-18 at 13:14 ┆ 2d 33m ago     │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ ibuprofen   ┆ 2x     ┆ 2025-04-18 at 17:57 ┆ 1d 19h 50m ago │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ paracetamol ┆ 1x     ┆ 2025-04-18 at 23:44 ┆ 1d 14h 3m ago  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ ibuprofen   ┆ 1x     ┆ 2025-04-19 at 07:46 ┆ 1d 6h 1m ago   │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ ibuprofen   ┆ 2x     ┆ 2025-04-19 at 14:12 ┆ 23h 35m ago    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ ibuprofen   ┆ 2x     ┆ 2025-04-19 at 20:15 ┆ 17h 32m ago    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ ibuprofen   ┆ 2x     ┆ 2025-04-20 at 08:45 ┆ 5h 2m ago      │
╰─────────────┴────────┴─────────────────────┴────────────────╯
╭─────────────┬──────────────╮
│ Name        ┆ Total Dosage │
╞═════════════╪══════════════╡
│ paracetamol ┆ 1            │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ ibuprofen   ┆ 10           │
╰─────────────┴──────────────╯
```

Often you just want to know how much you've taken in the last 24h as that is a common cycle with the medicine:
```sh
❯ medicine list
Only showing the last 24h:
╭───────────┬────────┬─────────────────────┬─────────────╮
│ Name      ┆ Dosage ┆ Time Taken          ┆ Since       │
╞═══════════╪════════╪═════════════════════╪═════════════╡
│ ibuprofen ┆ 2x     ┆ 2025-04-19 at 14:12 ┆ 23h 36m ago │
├╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ ibuprofen ┆ 2x     ┆ 2025-04-19 at 20:15 ┆ 17h 33m ago │
├╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ ibuprofen ┆ 2x     ┆ 2025-04-20 at 08:45 ┆ 5h 3m ago   │
╰───────────┴────────┴─────────────────────┴─────────────╯
╭───────────┬──────────────╮
│ Name      ┆ Total Dosage │
╞═══════════╪══════════════╡
│ ibuprofen ┆ 6            │
╰───────────┴──────────────╯
```
