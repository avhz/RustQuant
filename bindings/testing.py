from pprint import pprint as print
from datetime import date

from RustQuant.data import (
    Curve,
    CurveType,
    InterpolationMethod,
)
from RustQuant.time import (
    Calendar,
    Market,
)


dates = [
    date(2026, 1, 1),
    date(2027, 1, 2),
    date(2028, 1, 3),
    date(2029, 1, 4),
    date(2030, 1, 5),
]
rates = [
    0.01,
    0.015,
    0.012,
    0.014,
    0.013,
]

crv = Curve(dates, rates, CurveType.Spot, InterpolationMethod.Linear)
print(crv.get_rate(date(2026, 6, 1)))


new_dates = [
    date(2026, 6, 1),
    date(2026, 6, 2),
    date(2026, 6, 3),
    date(2026, 6, 4),
    date(2026, 6, 5),
]

cal = Calendar(Market.Australia)
print(cal.__dir__())
cal.market()
cal.extra_holidays()
cal.is_business_day(date(2023, 1, 3))
cal.add_holiday(date(2023, 1, 6))
