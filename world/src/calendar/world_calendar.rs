use calendar::WorldTime;

/// This is the number of days in a months,
/// number of months in a year,
/// and so on.
///
/// It's the data to describe how to work time
/// in our world.
///
/// This is not a representation of time it's
/// self. This won't hold the current date,
/// or the current time.
///
/// Time is represented in seconds. You pass this
/// into the calendar and it will give you a breakdown
/// of the time.
/// 
pub trait WorldCalendar {
    /// Given a numerical time stamp, like a unix time, this will return a more
    /// laid out version of that. A version with the exact date, hours, etc.
    fn get_time(
        &self,
        time: u32,
    ) -> WorldTime;
}
