#!/bin/bash
(cd backend ; cargo watch -c -w src -x run &)
(cd frontend ; trunk serve --open &)
systemctl start postgresql
