use crate::ec;
use crate::helper::resp::HttpErrorData;

#[catch(401)]
pub fn unauthorized() -> HttpErrorData {
    fail!(ec::Unauthorized)
}

#[catch(403)]
pub fn forbidden() -> HttpErrorData {
    fail!(ec::Forbidden)
}

#[catch(404)]
pub fn notfound() -> HttpErrorData {
    fail!(ec::NotFound)
}
