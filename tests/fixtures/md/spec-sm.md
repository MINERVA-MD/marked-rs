---
layout: page
title: CRDT - The Xi Text Engine
site_nav_category_order: 206
is_site_nav_category2: true
site_nav_category: docs
---

<!-- See https://www.figma.com/file/UGOAcpKR5WIP81t3DGPIP2dR/CRDT-Merge-Diagrams for the source of the diagrams -->

This document contains a detailed description of the data structures and operations Xi uses for text. These data structures and the `merge` operation also form a Conflict-free Replicated Data Type (CRDT). It being a CRDT allows Xi to be used for concurrent editing of text on multiple devices, it can merge edits, including those made offline, between multiple devices and converge on a consistent document that includes all changes.

Beyond synchronizing text, these data structures and operations allow Xi to handle asynchronous editing of the text from plugins, support undo and redo, and allow incremental updating of editor state and the view based on differences between revisions.

Many of these data structures and operations have been in Xi for a while but they've recently been heavily overhauled and extended as part of [a project](https://github.com/xi-editor/xi-editor/issues/250) that added multi-device syncing support via the CRDT merge operation. This was done for [use on the Fuchsia operating system](https://fuchsia.googlesource.com/topaz/+/master/bin/xi/), where it uses [Ledger](https://fuchsia.googlesource.com/peridot/+/HEAD/docs/ledger) to synchronize documents between devices.

What follows is both a description of a data structure for text and a code tour of the `xi-rope` crate. It describes the actual Rust data structures and algorithms used, because the primary novelty and difficulty of this CRDT is in the optimized representation that allows for better time and memory complexity. If you want an overview of the motivation behind using a CRDT and a conceptual description of what the CRDT does see [`crdt.md`](crdt.md). The intended audience is anyone interested in implementing CRDTs, anyone who wants to work on Xi, or just anyone curious enough.

## Table of Contents

- [Motivation](#motivation): Why Xi's CRDT is the way it is.
- [Representation](#representation): Describes the representation Xi uses to implement the CRDT in a memory and time efficient way.
- [Operations](#operations): Describes all the operations implemented on the representation to allow it to support undo, asynchronous edits, distributed synchronization and more.
    - [Engine::merge](#enginemerge): Description of the CRDT merge operation used for multi-device syncing.

## Motivation

The Xi CRDT attempts to have a number of properties that many other asynchronous text editing solutions don't:

- Usable without a central server: Even though the Fuchsia Ledger does sync to a cloud server, it can't actually inspect the data or perform operations, so conflict resolution must be possible on every device independently.
- Support large documents: The memory complexity of the representation and the time complexity of common operations should be low enough to support very large text documents like books or long code files.
- Support long histories: Similarly, documents that have gone through many edits should be efficient to edit both in time and memory.

As of the time this document was written, it satisfies all of these properties to some extent, but some operations and representations are not as memory and time efficient as we'd like. However, everything has been designed with a few more key optimizations in mind. Those optimizations should bring the memory and time complexity down to where we want.

### Transform Property 2 (TP2) and Operational Transforms

[Operational Transformation (OT)](https://en.wikipedia.org/wiki/Operational_transformation) is a common way to implement asynchronous text editing. It works by sending *operations* like inserts and deletes between peers and transforming them to apply to the current text. Unfortunately many implementations of OT have a problem where they don't always preserve ordering when text is deleted.

For example see the following diagram showing 3 peers sending edits between each other ending up in an inconsistent state. The arrows represent operations being sent asynchronously between devices in a peer-to-peer editing system based on OT, with time progressing downward. Whenever an edit is made the operation is sent to all other peers, but due to asynchronous communication they can be arbitrarily delayed. When an operation arrives at a peer it is transformed and applied to the current text. For clarity, not all arrows are shown, but you can imagine that the arrival of missing sends just got delayed past the end of the diagram.

![TP2 Problem](img/tp2.png)

Acting consistently in cases like this is called having "Transform Property 2" (see [Operational Transformation in Real-Time Group Editors:
Issues, Algorithms, and Achievements](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.53.933&rep=rep1&type=pdf) by Sun & Ellis, 1998). One approach to the problem is to serialize all edits through a central server which does all the transformation. With this approach, the ordering relative to deleted text may not be preserved, but at least all clients will converge to the same state. This is what Google Docs and many other collaborative editing systems do.

Xi avoids this problem by using "tombstones" (see [Tombstone Transformation Functions for Ensuring Consistency in Collaborative Editing Systems](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.103.2679&rep=rep1&type=pdf)), which leave deleted characters in the representation so that ordering can be preserved. This will be described in detail later.