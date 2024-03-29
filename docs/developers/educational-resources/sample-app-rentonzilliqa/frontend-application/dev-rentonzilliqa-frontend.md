---
id: dev-rentonzilliqa-frontend
title: Introduction to the frontend
keywords:
  - react
  - rentonzilliqa
  - frontend
description: Creating the RentOnZilliqa frontend application
---

---

In this section, we will build the frontend application for accessing
RentOnZilliqa.

## Technology

The frontend is built using
[Create React App](https://github.com/facebook/create-react-app) with
[TypeScript](https://www.typescriptlang.org) enabled. We rely on
[Tailwind CSS](https://tailwindcss.com) for styling the application. The setup
for these is freely available on the respective documentations.

Using [ZilPay](https://zilpay.io), we will connect the frontend elements to the
transitions and state of the Smart Contract.

## Pages

The frontend will consist of a homepage which shows all the available listings.
Details about each listing will be displayed on a separate listing page.

### Listings Page

This page lists all the houses that have been posted on the platform. For users
with a host account, it also shows the listings managed by them.

| <img alt="Listings Page" width="1600" src="../../../../../assets/img/dev-dapps/rentonzilliqa/listings.png" /> |
| ------------------------------------------------------------------------------------------------------------- |

<br />

### Individual Listing Page

This page presents the details about the listing. Users can make a reservation
for the listing from this page.

| <img alt="Individual Listing Page" width="1600" src="../../../../../assets/img/dev-dapps/rentonzilliqa/listing-1.png" /> |
| ------------------------------------------------------------------------------------------------------------------------ |
| <img alt="Individual Listing Page" width="1600" src="../../../../../assets/img/dev-dapps/rentonzilliqa/listing-2.png" /> |

<br />

## Modals

Most actions, including account and listing creation, booking, etc., will be
accessible via modals.

### Account creation and ZilPay

On the [Listings Page](#listings-page), the user can create an account. This is
done via a modal, which provides options to connect ZilPay and the form for
account creation.

| <img alt="Account Modal" width="1600" src="../../../../../assets/img/dev-dapps/rentonzilliqa/account.png" /> |
| ------------------------------------------------------------------------------------------------------------ |

<br />

### Creating and Managing Listings

From the [Listings Page](#listings-page), a host user can create listings and
manage existing listings via modals.

| <img alt="Create Listing Modal" width="1600" src="../../../../../assets/img/dev-dapps/rentonzilliqa/create-listing.png" /> |
| -------------------------------------------------------------------------------------------------------------------------- |
| <img alt="Update Listing Modal" width="1600" src="../../../../../assets/img/dev-dapps/rentonzilliqa/update-listing.png" /> |

## Building the Frontend

In the coming sections, we will build the frontend in the following stages:

- [Components](../scilla-contract/dev-rentonzilliqa-library.md)
- [Scripting](dev-rentonzilliqa-scripting.md)
- [Modals](dev-rentonzilliqa-modals.md)
- [Pages](dev-rentonzilliqa-pages.md)

## Built by Quinence

!["Quinence Logo"](https://quinence.com/favicon-196x196.png)

[Quinence - Digital product specialists from Singapore](https://quinence.com).
