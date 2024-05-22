# MVP

At first lets define what features should it have and then pick MVP.

- add new list
- share list via link, it should point to web app & mobile app (PWA?)
- scan something e.g. pdf/photo (at first)
- add your preferred stores, the ap should create optimal route, e.g. it should predict how long you will spend in store based on avg times or if enabled based on your analytics, then it should predict how long will it take you to make from one store to 2nd
- some reddit user said that it should have (there was no - option) if somethin from the list was missing so the checkboxes are three state
- the app should organize the list based on products occurance in store
- the app should recognize the available discounts and tell user about them (option to opt out from)

## Models

There is one pattern defined meaning [number] is feature priority

Product:

- name
- category[]

List:

- name, give it a name! by default todays date
- entry is: string | Product + quantity
- end -> calculate time spent + time between different things
- select latest shared people -> send notification
- create new based of this

ListEntry (define or cpy from Product if found automatically):

- checked_time -> it will later define the store path (avg), or user path (can be opted out)
- help by latest selected
- add user settings if entries should go to bottom of the list when checked

Store:

- [9]location
- [3] (google maps api) - select/autofill from google maps api
- name

User:

- anonymous (mobile/web)
- some oauth
- location
- preferred store(s)

The app allows for anonymous user access -> on mobile by fetching endpoint with device id, the user can create account at any time and they should be asked if they want to import the data or access_them later (there should be relation for this)

## Flows

### User creates list

1. User clicks button on the right
2.
